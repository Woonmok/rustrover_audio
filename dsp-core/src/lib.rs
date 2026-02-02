#![cfg_attr(not(feature = "std"), no_std)]

pub mod analyzer_advanced;
pub mod magnetic_eq;
pub mod processor_trait;
pub mod riaa_eq_advanced;
pub mod parallel_filter_advanced;
pub mod biquad_wrapper;
pub mod limiter;

// Re-exports
pub use analyzer_advanced::VelocityAnalyzer;
pub use magnetic_eq::MagneticEQ;
pub use processor_trait::{Processor, ProcessParams, mix_dry_wet, db_to_linear};
pub use riaa_eq_advanced::RIAAEQAdvanced;
pub use parallel_filter_advanced::ParallelFilterAdvanced;
pub use biquad_wrapper::BiquadWrapper;
pub use limiter::Limiter;

/// Legacy API support for plugins

#[derive(Clone, Debug)]
pub struct DspParams {
    pub drive_db: f32,
}

impl DspParams {
    pub fn new(drive_db: f32) -> Self {
        Self { drive_db }
    }
}

/// Legacy process_block function
pub fn process_block(
    input: &[f32],
    output: &mut [f32],
    _sample_rate: f32,
    params: &DspParams
) {
    let drive = db_to_linear_simple(params.drive_db);
    for (in_sample, out_sample) in input.iter().zip(output.iter_mut()) {
        let x = *in_sample * drive;
        let y = soft_clip(x);
        *out_sample = y;
    }
}

/// 기본 soft clipping 함수
#[inline]
pub fn soft_clip(x: f32) -> f32 {
    if x.abs() < 1.0 {
        x - (x.powi(3)) / 3.0
    } else {
        x.signum() * (2.0 / 3.0)
    }
}

/// dB to Linear 변환
#[inline]
pub fn db_to_linear_simple(db: f32) -> f32 {
    #[cfg(feature = "std")]
    {
        10.0_f32.powf(db / 20.0)
    }

    #[cfg(not(feature = "std"))]
    {
        use libm::pow10f;
        pow10f(db / 20.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_soft_clip_zero() {
        assert_eq!(soft_clip(0.0), 0.0);
    }

    #[test]
    fn test_soft_clip_small() {
        let result = soft_clip(0.5);
        assert!(result.abs() < 0.5);
    }

    #[test]
    fn test_soft_clip_large() {
        let result = soft_clip(5.0);
        assert!(result.abs() < 1.0);
    }

    #[test]
    fn test_process_block() {
        let input = vec![0.5; 10];
        let mut output = vec![0.0; 10];
        let params = DspParams::new(6.0);
        process_block(&input, &mut output, 48000.0, &params);
        assert!(output[0] != 0.0);
    }
}

