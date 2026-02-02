#![cfg_attr(not(feature = "std"), no_std)]

pub struct DspParams {
    pub drive_db: f32,
}

impl DspParams {
    pub fn new() -> Self {
        Self { drive_db: 0.0 }
    }
}

impl Default for DspParams {
    fn default() -> Self {
        Self::new()
    }
}

pub fn process_block(
    input: &[f32],
    output: &mut [f32],
    _sample_rate: f32,
    params: &DspParams
) {
    let drive = db_to_linear(params.drive_db);

    for (in_sample, out_sample) in input.iter().zip(output.iter_mut()) {
        let x = *in_sample * drive;
        let y = soft_clip(x);
        *out_sample = y;
    }
}

#[inline]
fn db_to_linear(db: f32) -> f32 {
    #[cfg(feature = "std")]
    {
        10.0_f32.powf(db / 20.0)
    }
    #[cfg(not(feature = "std"))]
    {
        libm::powf(10.0, db / 20.0)
    }
}

#[inline]
fn soft_clip(x: f32) -> f32 {
    const THRESHOLD: f32 = 1.0;
    if x > THRESHOLD {
        THRESHOLD + (x - THRESHOLD) / (1.0 + (x - THRESHOLD).abs())
    } else if x < -THRESHOLD {
        -THRESHOLD + (x + THRESHOLD) / (1.0 + (x + THRESHOLD).abs())
    } else {
        x
    }
}
