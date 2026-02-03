/// RIAA EQ - 고급 샘플레이트별 정확한 구현

/// RIAA EQ 계수 (biquad)
pub struct RIAACoefficients {
    pub b0: f32,
    pub b1: f32,
    pub b2: f32,
    pub a1: f32,
    pub a2: f32,
}

impl RIAACoefficients {
    /// 44.1kHz RIAA 계수
    pub fn riaa_44100() -> Self {
        // RIAA 곡선: 44.1kHz에 맞춘 biquad 계수
        Self {
            b0: 0.8831,
            b1: 0.0,
            b2: -0.8831,
            a1: -1.9663,
            a2: 0.9665,
        }
    }

    /// 48kHz RIAA 계수
    pub fn riaa_48000() -> Self {
        // RIAA 곡선: 48kHz에 맞춘 biquad 계수
        Self {
            b0: 0.8756,
            b1: 0.0,
            b2: -0.8756,
            a1: -1.9680,
            a2: 0.9742,
        }
    }

    /// 임의 샘플레이트에서 RIAA 계수 계산
    pub fn calculate(sample_rate: u32) -> Self {
        match sample_rate {
            44100 => Self::riaa_44100(),
            48000 => Self::riaa_48000(),
            _ => {
                // 가장 가까운 샘플레이트로 근사
                if (sample_rate as i32 - 44100).abs() < (sample_rate as i32 - 48000).abs() {
                    Self::riaa_44100()
                } else {
                    Self::riaa_48000()
                }
            }
        }
    }
}

/// RIAA EQ 필터 (Direct Form II Transposed)
pub struct RIAAEQAdvanced {
    coeffs: RIAACoefficients,
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
}

impl RIAAEQAdvanced {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            coeffs: RIAACoefficients::calculate(sample_rate),
            x1: 0.0,
            x2: 0.0,
            y1: 0.0,
            y2: 0.0,
        }
    }

    /// Direct Form II Transposed 구조
    pub fn process(&mut self, x: f32) -> f32 {
        let y = self.coeffs.b0 * x + self.y1;
        self.y1 = self.coeffs.b1 * x - self.coeffs.a1 * y + self.y2;
        self.y2 = self.coeffs.b2 * x - self.coeffs.a2 * y;
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
    fn test_riaa_44100_coeffs() {
        let coeffs = RIAACoefficients::riaa_44100();
        assert!(coeffs.b0.abs() < 1.0);
        assert!(coeffs.a1.abs() < 2.0);
    }

    #[test]
    fn test_riaa_eq_process() {
        let mut eq = RIAAEQAdvanced::new(44100);
        let output = eq.process(0.5);
        assert!(!output.is_nan());
    }

    #[test]
    fn test_riaa_eq_reset() {
        let mut eq = RIAAEQAdvanced::new(48000);
        eq.process(1.0);
        eq.reset();
        assert_eq!(eq.y1, 0.0);
        assert_eq!(eq.y2, 0.0);
    }
}
