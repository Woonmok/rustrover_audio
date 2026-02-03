/// Limiter - 피크 클리핑 방지

pub struct Limiter {
    threshold: f32,
    release_time: f32,
    envelope: f32,
}

impl Limiter {
    pub fn new(threshold: f32, release_time: f32) -> Self {
        Self {
            threshold: threshold.clamp(0.0, 1.0),
            release_time: release_time.max(0.001),
            envelope: 0.0,
        }
    }

    pub fn process(&mut self, input: f32) -> f32 {
        let abs_input = input.abs();

        if abs_input > self.threshold {
            self.envelope = abs_input;
        } else {
            let release_coeff = (-1.0 / self.release_time).exp();
            self.envelope = self.envelope * release_coeff;
        }

        if self.envelope > 0.0 {
            input * (self.threshold / self.envelope)
        } else {
            input
        }
    }

    pub fn reset(&mut self) {
        self.envelope = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_limiter_creation() {
        let limiter = Limiter::new(0.95, 0.1);
        assert_eq!(limiter.threshold, 0.95);
    }

    #[test]
    fn test_limiter_clipping() {
        let mut limiter = Limiter::new(0.9, 0.05);
        let output = limiter.process(2.0);
        assert!(output.abs() <= 1.0);
    }
}
