/// Parallel HF Recovery 필터
/// 약한 신호의 고주파를 복구하는 병렬 필터

pub struct ParallelFilterAdvanced {
    intensity: f32, // 0.0-1.0
    state: f32,
}

impl ParallelFilterAdvanced {
    pub fn new(intensity: f32) -> Self {
        Self {
            intensity: intensity.clamp(0.0, 1.0),
            state: 0.0,
        }
    }

    /// 병렬 필터 처리 (Dry + Wet 크로스페이드)
    pub fn process(&mut self, input: f32) -> f32 {
        // HF 부스트 경로 (간단한 high-pass 근사)
        let hf_boosted = input + (input - self.state) * 0.5;
        self.state = input;

        // Dry/Wet 믹싱
        input * (1.0 - self.intensity) + hf_boosted * self.intensity
    }

    pub fn set_intensity(&mut self, intensity: f32) {
        self.intensity = intensity.clamp(0.0, 1.0);
    }

    pub fn reset(&mut self) {
        self.state = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_filter_creation() {
        let filter = ParallelFilterAdvanced::new(0.5);
        assert_eq!(filter.intensity, 0.5);
    }

    #[test]
    fn test_parallel_filter_process() {
        let mut filter = ParallelFilterAdvanced::new(0.5);
        let output = filter.process(0.5);
        assert!(!output.is_nan());
    }
}
