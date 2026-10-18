//! PipeWire audio output.
//!
//! PipeWire drives streams from its own main loop, so this output runs that
//! loop on a dedicated thread. The front end hands samples over through a
//! single-producer single-consumer ring buffer, which the stream's process
//! callback drains on the audio thread.

use std::io::Cursor;
use std::sync::mpsc;
use std::thread::{self, JoinHandle};

// The child module shadows the crate name, so imports are rooted with `::`.
use ::pipewire as pw;
use pw::properties::properties;
use pw::spa::param::audio::{AudioFormat, AudioInfoRaw, MAX_CHANNELS};
use pw::spa::pod::{serialize::PodSerializer, Object, Pod, Value};
use pw::spa::utils::Direction;

use super::AudioOutput;
use crate::apu::SAMPLE_RATE;

/// Number of channels of the interleaved PCM sent to the stream.
const CHANNELS: usize = 2;

/// Size of one PCM sample in bytes.
const SAMPLE_SIZE: usize = std::mem::size_of::<i16>();

/// Bytes one stereo frame occupies in the stream buffers.
const FRAME_SIZE: usize = SAMPLE_SIZE * CHANNELS;

/// Number of stereo frames the ring buffer between the emulation thread and
/// the stream can hold, roughly 85 ms at 48 kHz. Samples that do not fit are
/// dropped so that playback latency stays bounded.
const BUFFER_FRAMES: usize = 4096;

/// Number of stereo frames of silence pushed into the ring buffer on creation,
/// giving the stream a cushion to play from while the first emulated frames
/// are produced.
const PRIME_FRAMES: usize = BUFFER_FRAMES / 2;

/// Plays the samples produced by the APU through a PipeWire stream.
pub struct PipeWireOutput {
    /// Writing end of the ring buffer; the stream reads from the other end.
    samples: rtrb::Producer<i16>,

    /// Asks the loop thread to quit when the output is dropped.
    quit: pw::channel::Sender<()>,

    thread: Option<JoinHandle<()>>,
}

impl PipeWireOutput {
    /// Opens a playback stream on the default PipeWire sink.
    ///
    /// Blocks until the stream has been created, so that setup errors (for
    /// example a missing PipeWire server) can be reported synchronously.
    pub fn new() -> Result<Self, String> {
        let (mut samples, consumer) = rtrb::RingBuffer::new(BUFFER_FRAMES * CHANNELS);

        let silence = vec![0; PRIME_FRAMES * CHANNELS];
        let (_, leftover) = samples.push_partial_slice(&silence);
        debug_assert!(
            leftover.is_empty(),
            "the ring buffer must fit the samples it is primed with"
        );

        let (quit_sender, quit_receiver) = pw::channel::channel::<()>();
        let (ready_sender, ready_receiver) = mpsc::channel::<Result<(), String>>();

        let thread = thread::Builder::new()
            .name("pipewire".to_owned())
            .spawn(move || run(consumer, quit_receiver, &ready_sender))
            .map_err(|err| format!("failed to spawn the PipeWire thread: {err}"))?;

        match ready_receiver.recv() {
            Ok(Ok(())) => Ok(Self {
                samples,
                quit: quit_sender,
                thread: Some(thread),
            }),
            Ok(Err(err)) => {
                let _ = thread.join();
                Err(err)
            }
            Err(_) => {
                let _ = thread.join();
                Err("the PipeWire thread exited before the stream was created".to_owned())
            }
        }
    }
}

impl AudioOutput for PipeWireOutput {
    fn play(&mut self, samples: &[i16]) {
        let (_, leftover) = self.samples.push_partial_slice(samples);

        if !leftover.is_empty() {
            // The stream is behind (for example because it was suspended).
            // Dropping the samples that do not fit keeps the latency bounded.
            log::trace!(
                "dropped {} audio samples, the PipeWire buffer is full",
                leftover.len()
            );
        }
    }
}

impl Drop for PipeWireOutput {
    fn drop(&mut self) {
        let _ = self.quit.send(());

        if let Some(thread) = self.thread.take() {
            if thread.join().is_err() {
                log::warn!("the PipeWire thread panicked");
            }
        }
    }
}

/// Runs the main loop that drives the stream until the output is dropped.
///
/// Every PipeWire object is created and destroyed on this thread, as they are
/// not `Send`. The front end only talks to the ring buffer and to the quit
/// channel.
fn run(
    consumer: rtrb::Consumer<i16>,
    quit_receiver: pw::channel::Receiver<()>,
    ready_sender: &mpsc::Sender<Result<(), String>>,
) {
    let session = match Session::new(consumer) {
        Ok(session) => session,
        Err(err) => {
            let _ = ready_sender.send(Err(err));
            return;
        }
    };

    // Quitting the main loop is what shuts the thread down.
    let _quit = quit_receiver.attach(session.mainloop.loop_(), {
        let mainloop = session.mainloop.clone();
        move |_| mainloop.quit()
    });

    if ready_sender.send(Ok(())).is_err() {
        // The front end is gone, so there is nothing left to play for.
        log::debug!("audio output dropped before the PipeWire stream was ready");
        return;
    }

    session.mainloop.run();
    log::debug!("PipeWire thread exited");
}

/// The PipeWire objects backing one playback stream.
///
/// Fields are dropped in declaration order, so the listener detaches from the
/// stream before the stream is torn down, and the loop driving both is
/// destroyed last. The stream keeps the core and the context alive.
struct Session {
    _listener: pw::stream::StreamListener<rtrb::Consumer<i16>>,
    _stream: pw::stream::StreamRc,
    mainloop: pw::main_loop::MainLoopRc,
}

impl Session {
    /// Creates a stream playing [`SAMPLE_RATE`] stereo 16-bit PCM and connects
    /// it to the default sink.
    fn new(consumer: rtrb::Consumer<i16>) -> Result<Self, String> {
        let mainloop = pw::main_loop::MainLoopRc::new(None)
            .map_err(|err| format!("failed to create the PipeWire main loop: {err}"))?;
        let context = pw::context::ContextRc::new(&mainloop, None)
            .map_err(|err| format!("failed to create the PipeWire context: {err}"))?;
        let core = context
            .connect_rc(None)
            .map_err(|err| format!("failed to connect to the PipeWire server: {err}"))?;

        let stream = pw::stream::StreamRc::new(
            core,
            "rusty_fuugbemu",
            properties! {
                *pw::keys::MEDIA_TYPE => "Audio",
                *pw::keys::MEDIA_CATEGORY => "Playback",
                *pw::keys::MEDIA_ROLE => "Game",
                *pw::keys::MEDIA_NAME => "RustyFuuGBemu",
                *pw::keys::APP_NAME => "RustyFuuGBemu",
                *pw::keys::AUDIO_CHANNELS => "2",
                *pw::keys::AUDIO_RATE => "48000",
            },
        )
        .map_err(|err| format!("failed to create the PipeWire stream: {err}"))?;

        let _listener = stream
            .add_local_listener_with_user_data(consumer)
            .state_changed(|_, _, old, new| {
                log::debug!("PipeWire stream state changed: {old:?} -> {new:?}");
            })
            .process(play_samples)
            .register()
            .map_err(|err| format!("failed to register the PipeWire stream callbacks: {err}"))?;

        let format = serialize_format()?;
        let pod =
            Pod::from_bytes(&format).ok_or("failed to parse the serialized audio stream format")?;
        let mut params = [pod];

        stream
            .connect(
                Direction::Output,
                None,
                pw::stream::StreamFlags::AUTOCONNECT
                    | pw::stream::StreamFlags::MAP_BUFFERS
                    | pw::stream::StreamFlags::RT_PROCESS,
                &mut params,
            )
            .map_err(|err| format!("failed to connect the PipeWire stream: {err}"))?;

        Ok(Self {
            _listener,
            _stream: stream,
            mainloop,
        })
    }
}

/// Serializes the format the APU produces, interleaved 16-bit little endian
/// stereo PCM, into the Pod the stream advertises during negotiation.
fn serialize_format() -> Result<Vec<u8>, String> {
    let mut audio_info = AudioInfoRaw::new();
    audio_info.set_format(AudioFormat::S16LE);
    audio_info.set_rate(SAMPLE_RATE);
    audio_info.set_channels(CHANNELS as u32);

    let mut position = [0; MAX_CHANNELS];
    position[0] = pw::spa::sys::SPA_AUDIO_CHANNEL_FL;
    position[1] = pw::spa::sys::SPA_AUDIO_CHANNEL_FR;
    audio_info.set_position(position);

    let (cursor, _) = PodSerializer::serialize(
        Cursor::new(Vec::new()),
        &Value::Object(Object {
            type_: pw::spa::sys::SPA_TYPE_OBJECT_Format,
            id: pw::spa::sys::SPA_PARAM_EnumFormat,
            properties: audio_info.into(),
        }),
    )
    .map_err(|err| format!("failed to serialize the audio stream format: {err}"))?;

    Ok(cursor.into_inner())
}

/// Fills one stream buffer with samples from the ring buffer, padding the tail
/// with silence when the ring buffer runs dry.
fn play_samples(stream: &pw::stream::Stream, consumer: &mut rtrb::Consumer<i16>) {
    let Some(mut buffer) = stream.dequeue_buffer() else {
        return;
    };

    // The graph asks for the number of frames it needs this cycle, which can
    // be smaller than the mapped buffer.
    let requested = buffer.requested() as usize;

    let datas = buffer.datas_mut();
    let Some(data) = datas.first_mut() else {
        return;
    };
    let Some(mapped) = data.data() else {
        return;
    };

    let frames = requested.min(mapped.len() / FRAME_SIZE);
    let written = fill_stream_buffer(&mut mapped[..frames * FRAME_SIZE], consumer);

    let chunk = data.chunk_mut();
    *chunk.offset_mut() = 0;
    *chunk.stride_mut() = FRAME_SIZE as i32;
    *chunk.size_mut() = written as u32;
}

/// Copies samples from the ring buffer into `mapped`, padding the tail with
/// silence when the ring buffer runs dry. Samples are stored as little endian
/// 16-bit integers, matching the negotiated `S16LE` stream format. Returns the
/// number of bytes written into `mapped`.
fn fill_stream_buffer(mapped: &mut [u8], consumer: &mut rtrb::Consumer<i16>) -> usize {
    let (chunks, _) = mapped.as_chunks_mut::<SAMPLE_SIZE>();

    for chunk in chunks.iter_mut() {
        match consumer.pop() {
            Ok(sample) => *chunk = sample.to_le_bytes(),
            Err(_) => *chunk = [0; SAMPLE_SIZE],
        }
    }

    chunks.len() * SAMPLE_SIZE
}

#[path = "pipewire_test.rs"]
#[cfg(test)]
mod test;
