use fon::chan::{Ch32, Ch16};
use fon::pos::{Mono};
use fon::Audio;
use rand::{Rng};
use rodio::buffer::SamplesBuffer;

pub fn sawtooth_wave(time: usize, freq: i32) -> SamplesBuffer<i16> {
    let mut a = Audio::<Ch32, 1>::with_silence(48_000, 48_000 * time);
    let mut counter = 0.0;
    let scale = 48_000.0 / freq as f32;
    for f in a.iter_mut() {
        f[Mono] = counter.into();
        counter += 1.0/scale as f32;
        counter %= 1.0;
    }

    let mut audio = Audio::<Ch16, 1>::with_audio(48_000, &a);

    return SamplesBuffer::new(1, 48_000, audio.as_i16_slice());
}

pub fn square_wave(time: usize, freq: i32) -> SamplesBuffer<i16> {
    let mut a = Audio::<Ch32, 1>::with_silence(48_000, 48_000 * time);
    let mut is_up = true;
    let mut counter = 0.0;
    let scale = 48_000.0 / freq as f32;
    for f in a.iter_mut() {
        let prev_counter = counter;
        f[Mono] = if is_up { 1.0.into() } else { 0.0.into() };
        counter += 1.0/scale;
        counter %= 0.5 as f32;
        if counter < prev_counter { is_up = !is_up }
    }

    let mut audio = Audio::<Ch16, 1>::with_audio(48_000, &a);

    return SamplesBuffer::new(1, 48_000, audio.as_i16_slice());
}

pub fn triangle_wave(time: usize, freq: i32) -> SamplesBuffer<i16> {
    let mut a = Audio::<Ch32, 1>::with_silence(48_000, 48_000 * time);
    let mut is_up = true;
    let mut counter = 0.0;
    let scale = 48_000.0 / freq as f32;
    for f in a.iter_mut() {
        let prev_counter = counter;
        f[Mono] = if is_up { counter.into() } else { (1.0 - counter).into() };
        counter += 1.0/scale;
        counter %= 0.5 as f32;
        if counter < prev_counter { is_up = !is_up }
    }

    let mut audio = Audio::<Ch16, 1>::with_audio(48_000, &a);

    return SamplesBuffer::new(1, 48_000, audio.as_i16_slice());
}

pub fn white_noise_wave(time: usize) -> SamplesBuffer<i16> {
    let mut audio = Audio::<Ch16, 1>::with_silence(48_000, 48_000 * time);

    let mut rand = rand::thread_rng();

    for f in audio.iter_mut() {
        f[Mono] = rand.gen_range(0.0..=1.0).into();
    }

    return SamplesBuffer::new(1, 48_000, audio.as_i16_slice());
}

pub fn blank_wave(time: usize) -> SamplesBuffer<i16> {
    let mut audio = Audio::<Ch16, 1>::with_silence(48_000, 48_000 * time);

    return SamplesBuffer::new(1, 48_000, audio.as_i16_slice());
}