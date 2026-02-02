/// Biquad 필터 래퍼
/// 다목적 2차 IIR 필터

pub struct BiquadWrapper {
    b0: f32,
    b1: f32,
    b2: f32,
    a1: f32,
    a2: f32,
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
}

impl BiquadWrapper {
    pub fn new(b0: f32, b1: f32, b2: f32, a1: f32, a2: f32) -> Self {
        Self {
            b0,
            b1,
            b2,
            a1,
            a2,
            x1: 0.0,
            x2: 0.0,
            y1: 0.0,
            y2: 0.0,
        }
    }

    /// Lowpass 필터 계수 생성
    pub fn lowpass(cutoff: f32, q: f32) -> Self {
        let w = 2.0 * core::f32::consts::PI * cutoff;
        let sin_w = w.sin();
        let cos_w = w.cos();
        let alpha = sin_w / (2.0 * q);

        let b0 = (1.0 - cos_w) / 2.0;
        let b1 = 1.0 - cos_w;
        let b2 = (1.0 - cos_w) / 2.0;
        let a0 = 1.0 + alpha;
        let a1 = -2.0 * cos_w;
        let a2 = 1.0 - alpha;

        Self::new(b0 / a0, b1 / a0, b2 / a0, a1 / a0, a2 / a0)
    }

    /// Direct Form II Transposed 구조
    pub fn process(&mut self, x: f32) -> f32 {
        let y = self.b0 * x + self.y1;
        self.y1 = self.b1 * x - self.a1 * y + self.y2;
        self.y2 = self.b2 * x - self.a2 * y;
        y
    }

    pub fn reset(&mut self) {
        self.x1 = 0.0;
        self.x2 = 0.0;
        self.y1 = 0.0;
        self.y2 = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_biquad_creation() {
        let biquad = BiquadWrapper::new(0.5, 0.0, 0.5, 0.0, 0.0);
        assert_eq!(biquad.b0, 0.5);
    }

    #[test]
    fn test_lowpass_creation() {
        let biquad = BiquadWrapper::lowpass(0.1, 1.0);
        assert!(!biquad.b0.is_nan());
    }
}
