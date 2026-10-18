//! Audio playback technologies.

mod pipewire;

pub use self::pipewire::PipeWireOutput;

/// Standardized buffer of interleaved stereo 16-bit PCM samples that the
/// emulator back end sends to the front end whenever a batch of audio data is
/// ready to be played.
pub type AudioData = Vec<i16>;

/// Interface implemented by every audio technology.
///
/// The application only depends on this trait, so audio back ends can be
/// swapped, for example through a command line argument, without changing the
/// code that drives the emulator. Only [`AudioData`] crosses the boundary
/// between the emulator back end and the audio output, so the back end stays
/// free of audio playing dependencies.
pub trait AudioOutput {
    /// Hands a batch of [`AudioData`] to the output device.
    ///
    /// Implementations buffer the samples internally and consume them at
    /// their own pace, so the call must not block while the batch plays.
    fn play(&mut self, samples: &[i16]);
}

/// Audio technology selected with the `--audio` command line argument.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, clap::ValueEnum)]
pub enum AudioBackend {
    /// Play back through a PipeWire stream.
    #[default]
    #[value(name = "pipewire")]
    PipeWire,
    /// Discard all audio data, producing no sound.
    None,
}

impl AudioBackend {
    /// Creates the selected audio output.
    ///
    /// Failing to open the selected technology (e.g. no PipeWire server is
    /// available) is not fatal: a warning is logged and a silent output is
    /// returned, so emulation continues without sound.
    pub fn create(self) -> Box<dyn AudioOutput> {
        match self {
            Self::PipeWire => match PipeWireOutput::new() {
                Ok(output) => Box::new(output),
                Err(err) => {
                    log::warn!(
                        "failed to open the PipeWire output, continuing without sound: {err}"
                    );
                    Box::new(NullOutput)
                }
            },
            Self::None => Box::new(NullOutput),
        }
    }
}

/// Audio output that discards every batch, producing no sound.
struct NullOutput;

impl AudioOutput for NullOutput {
    fn play(&mut self, _samples: &[i16]) {}
}
