/// Velocity 분석 및 신호 특성 추출
/// 다차원 점수: RMS + Crest Factor + Spectral Tilt

/// 다차원 velocity 분석기
pub struct VelocityAnalyzer {
    sample_rate: f32,
}

impl VelocityAnalyzer {
    pub fn new(sample_rate: f32) -> Self {
        Self { sample_rate }
    }

    /// RMS 계산
    pub fn calculate_rms(input: &[f32]) -> f32 {
        if input.is_empty() {
            return 0.0;
        }
        let sum_sq: f32 = input.iter().map(|&s| s * s).sum();
        (sum_sq / input.len() as f32).sqrt()
    }

    /// Crest Factor 계산 (Peak / RMS)
    pub fn calculate_crest_factor(input: &[f32]) -> f32 {
        let rms = Self::calculate_rms(input);
        if rms == 0.0 {
            return 0.0;
        }
        let peak = input.iter().map(|&s| s.abs()).fold(0.0f32, f32::max);
        peak / rms
    }

    /// Spectral Tilt 추정 (간단한 버전)
    /// Low-Frequency vs High-Frequency 비율
    pub fn calculate_spectral_tilt(input: &[f32]) -> f32 {
        if input.len() < 4 {
            return 0.5;
        }

        // 첫 1/4 (저주파)와 마지막 1/4 (고주파) 비교
        let quarter = input.len() / 4;
        let low_energy: f32 = input[..quarter]
            .iter()
            .map(|&s| s * s)
            .sum();
        let high_energy: f32 = input[input.len() - quarter..]
            .iter()
            .map(|&s| s * s)
            .sum();

        let total = low_energy + high_energy;
        if total == 0.0 {
            return 0.5;
        }
        low_energy / total
    }

    /// Velocity 점수 계산 (0.0-1.0)
    /// velocity = sigmoid(0.5*rms + 0.3*crest + 0.2*spectral_tilt)
    pub fn calculate_velocity(input: &[f32]) -> f32 {
        let rms = Self::calculate_rms(input);
        let crest = Self::calculate_crest_factor(input);
        let tilt = Self::calculate_spectral_tilt(input);

        // 정규화 (0-1)
        let rms_norm = (rms * 10.0).min(1.0); // 최대 0.1 -> 1.0
        let crest_norm = ((crest - 1.0) / 10.0).max(0.0).min(1.0); // 1-11 range
        let tilt_norm = tilt;

        // 가중 평균
        let score = 0.5 * rms_norm + 0.3 * crest_norm + 0.2 * tilt_norm;

        // Sigmoid 함수 (부드러운 0-1 범위)
        sigmoid(score * 2.0 - 1.0)
    }
}

/// Sigmoid 함수
#[inline]
fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + (-x).exp())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rms_calculation() {
        let signal = vec![0.5, -0.5, 0.5, -0.5];
        let rms = VelocityAnalyzer::calculate_rms(&signal);
        assert!((rms - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_crest_factor() {
        let signal = vec![0.5, 0.5, 0.5, 1.0]; // Peak=1.0, RMS≈0.612
        let crest = VelocityAnalyzer::calculate_crest_factor(&signal);
        assert!(crest > 1.0);
    }

    #[test]
    fn test_velocity_range() {
        let signal = vec![0.1; 100];
        let velocity = VelocityAnalyzer::calculate_velocity(&signal);
        assert!(velocity >= 0.0 && velocity <= 1.0);
    }
}
