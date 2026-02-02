/// 스텁 모듈들 - CLI 도구

pub mod args {
    use std::path::PathBuf;

    #[derive(Debug)]
    pub struct Args {
        pub input: Option<PathBuf>,
        pub output: Option<PathBuf>,
        pub preset: Option<String>,
        pub drywet: Option<f32>,
        pub report: bool,
        pub verbose: bool,
    }
}

pub mod presets {
    pub fn get_preset_drive(name: &str) -> f32 {
        match name {
            "vinyl" => 8.0,
            "warm" => 5.0,
            "clean" => 2.0,
            _ => 4.0,
        }
    }
}

pub mod report {
    pub fn generate_report(_processed_samples: &[f32]) -> String {
        "Report placeholder".to_string()
    }
}
