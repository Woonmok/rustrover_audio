/// 공통 DSP Processor 트레이트
/// 모든 DSP 처리 모듈이 구현해야 할 인터페이스

/// DSP 파라미터 구조
#[derive(Clone, Debug)]
pub struct ProcessParams {
    pub drive_db: f32,
    pub dry_wet: f32,
    pub intensity: f32,
}

impl ProcessParams {
    pub fn new() -> Self {
        Self {
            drive_db: 0.0,
            dry_wet: 0.5,
            intensity: 0.5,
        }
    }
}

impl Default for ProcessParams {
    fn default() -> Self {
        Self::new()
    }
}

/// 공통 Processor 트레이트
pub trait Processor {
    /// 오디오 블록 처리
    fn process(&mut self, input: &[f32], output: &mut [f32], params: &ProcessParams);

    /// 샘플 단위 처리
    fn process_sample(&mut self, input: f32, params: &ProcessParams) -> f32;

    /// 파라미터 업데이트
    fn update_params(&mut self, params: &ProcessParams) {}

    /// 리셋 (상태 초기화)
    fn reset(&mut self) {}

    /// 프로세서 이름
    fn name(&self) -> &str;
}

/// Dry/Wet 믹싱 헬퍼
pub fn mix_dry_wet(dry: f32, wet: f32, mix: f32) -> f32 {
    dry * (1.0 - mix) + wet * mix
}

/// dB를 Linear로 변환
#[inline]
pub fn db_to_linear(db: f32) -> f32 {
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
    fn test_process_params() {
        let params = ProcessParams::new();
        assert_eq!(params.drive_db, 0.0);
        assert_eq!(params.dry_wet, 0.5);
        assert_eq!(params.intensity, 0.5);
    }

    #[test]
    fn test_mix_dry_wet() {
        let result = mix_dry_wet(0.0, 1.0, 0.5);
        assert!((result - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_db_to_linear() {
        let linear = db_to_linear(0.0);
        assert!((linear - 1.0).abs() < 0.01);

        let linear_6db = db_to_linear(6.0);
        assert!(linear_6db > 1.0);
    }
}
