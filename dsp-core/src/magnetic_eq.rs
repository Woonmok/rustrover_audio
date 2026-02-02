/// Magnetic Tape Emulation
/// 자기 테이프의 비선형 특성 시뮬레이션

/// Magnetic Tape 에뮬레이터
pub struct MagneticEQ {
    saturation: f32,
    hardness: f32,
}

impl MagneticEQ {
    pub fn new(saturation: f32, hardness: f32) -> Self {
        Self {
            saturation: saturation.clamp(0.0, 1.0),
            hardness: hardness.clamp(0.0, 1.0),
        }
    }

    /// 자기 테이프 포화 곡선
    /// 매끄러운 클리핑으로 따뜻한 톤 생성
    pub fn process(&self, sample: f32) -> f32 {
        let drive = 1.0 + self.saturation * 5.0; // 1.0-6.0
        let driven = sample * drive;

        // Soft clipping with magnetic saturation curve
        let tape_saturation = self.tape_saturation(driven, self.hardness);

        // Makeup gain
        tape_saturation / (drive * 0.8)
    }

    /// 테이프 포화 곡선 (비선형 곡선)
    fn tape_saturation(&self, x: f32, hardness: f32) -> f32 {
        if x.abs() < 0.5 {
            // 선형 영역
            x
        } else {
            // 포화 영역: 하드니스에 따라 다른 곡선
            let sign = x.signum();
            let abs_x = x.abs();

            // 부드러운 포화 곡선
            let knee = 0.5 + hardness * 0.2; // 0.5-0.7
            let excess = (abs_x - knee).max(0.0);
            let saturation_factor = 1.0 - hardness * 0.3;

            let saturated = knee + excess * saturation_factor;
            sign * saturated.min(1.0 + hardness * 0.1)
        }
    }

    /// 매개변수 업데이트
    pub fn set_saturation(&mut self, saturation: f32) {
        self.saturation = saturation.clamp(0.0, 1.0);
    }

    pub fn set_hardness(&mut self, hardness: f32) {
        self.hardness = hardness.clamp(0.0, 1.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magnetic_eq_creation() {
        let eq = MagneticEQ::new(0.5, 0.3);
        assert_eq!(eq.saturation, 0.5);
        assert_eq!(eq.hardness, 0.3);
    }

    #[test]
    fn test_magnetic_eq_process() {
        let eq = MagneticEQ::new(0.5, 0.5);
        let input = 0.1;
        let output = eq.process(input);
        assert!(!output.is_nan());
        assert!(!output.is_infinite());
    }

    #[test]
    fn test_saturation_clipping() {
        let eq = MagneticEQ::new(0.9, 0.8);
        let large_input = 2.0;
        let output = eq.process(large_input);
        assert!(output.abs() <= 1.5); // 클리핑 확인
    }
}
