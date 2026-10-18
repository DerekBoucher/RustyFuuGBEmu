use super::{fill_stream_buffer, SAMPLE_SIZE};
use rtrb::RingBuffer;

#[test]
fn available_samples_are_written_as_little_endian_bytes() {
    let (mut producer, mut consumer) = RingBuffer::new(8);
    for sample in [1_i16, -2, 3, 4] {
        producer.push(sample).unwrap();
    }

    let mut mapped = [0_u8; 4 * SAMPLE_SIZE];
    let written = fill_stream_buffer(&mut mapped, &mut consumer);

    assert_eq!(written, mapped.len());
    assert_eq!(mapped, [1, 0, 0xFE, 0xFF, 3, 0, 4, 0]);
}

#[test]
fn samples_missing_from_the_ring_buffer_are_padded_with_silence() {
    let (mut producer, mut consumer) = RingBuffer::new(8);
    producer.push(1).unwrap();

    let mut mapped = [0xFF_u8; 2 * SAMPLE_SIZE];
    let written = fill_stream_buffer(&mut mapped, &mut consumer);

    assert_eq!(written, mapped.len());
    assert_eq!(mapped, [1, 0, 0, 0]);
}

#[test]
fn bytes_that_cannot_hold_a_sample_are_left_untouched() {
    let (_producer, mut consumer) = RingBuffer::new(8);

    let mut mapped = [0xF0_u8; 2 * SAMPLE_SIZE + 1];
    let written = fill_stream_buffer(&mut mapped, &mut consumer);

    // The trailing byte cannot hold a sample and is left untouched.
    assert_eq!(written, 2 * SAMPLE_SIZE);
    assert_eq!(mapped[2 * SAMPLE_SIZE], 0xF0);
}
