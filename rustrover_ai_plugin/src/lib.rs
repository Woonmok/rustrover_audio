// 2026형 VST3 플러그인 - 특허 기반 Velocity-Adaptive DSP
// Advanced FeatureAnalyzer + RIAA/Parallel Crossfade + Sample-Rate Specific

use nih_plug::prelude::*;
use dsp_core::{
    MagneticEQ,
    ParallelFilterAdvanced,
    RIAAEQAdvanced,
    VelocityAnalyzer,
    mix_dry_wet,
};
use std::{array, sync::Arc};

pub struct RustroverAiPlugin {
    params: Arc<PluginParams>,
    sample_rate: f32,
    channels: [ChannelDsp; 2],
}

struct ChannelDsp {
    riaa: RIAAEQAdvanced,
    parallel: ParallelFilterAdvanced,
}

impl ChannelDsp {
    fn new(sample_rate: u32) -> Self {
        Self {
            riaa: RIAAEQAdvanced::new(sample_rate),
            parallel: ParallelFilterAdvanced::new(0.0),
        }
    }

    fn reset(&mut self) {
        self.riaa.reset();
        self.parallel.reset();
    }
}

#[derive(Params)]
pub struct PluginParams {
    /// 프리셋 (Vinyl / Warm / Clean)
    #[id = "preset"]
    pub preset: EnumParam<PresetType>,

    /// 드라이브 (포화도 강도)
    #[id = "drive"]
    pub drive: FloatParam,

    /// Dry/Wet 믹스 비율
    #[id = "drywet"]
    pub drywet: FloatParam,

    /// RIAA Intensity (특허: 크로스페이드 방식)
    #[id = "riaa_intensity"]
    pub riaa_intensity: FloatParam,

    /// Parallel HF Recovery Mix (특허: Velocity < 0.5)
    #[id = "parallel_mix"]
    pub parallel_mix: FloatParam,

    /// Auto Velocity Analysis (On/Off)
    #[id = "auto_velocity"]
    pub auto_velocity: BoolParam,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresetType {
    Vinyl,
    Warm,
    Clean,
}

impl Enum for PresetType {
    fn variants() -> &'static [&'static str] {
        &["Vinyl", "Warm", "Clean"]
    }

    fn ids() -> Option<&'static [&'static str]> {
        Some(&["vinyl", "warm", "clean"])
    }

    fn to_index(self) -> usize {
        match self {
            PresetType::Vinyl => 0,
            PresetType::Warm => 1,
            PresetType::Clean => 2,
        }
    }

    fn from_index(index: usize) -> Self {
        match index {
            0 => PresetType::Vinyl,
            1 => PresetType::Warm,
            _ => PresetType::Clean,
        }
    }
}

impl Default for RustroverAiPlugin {
    fn default() -> Self {
        let sample_rate = 48000.0;
        Self {
            params: Arc::new(PluginParams::default()),
            sample_rate,
            channels: array::from_fn(|_| ChannelDsp::new(sample_rate as u32)),
        }
    }
}

impl Default for PluginParams {
    fn default() -> Self {
        Self {
            preset: EnumParam::new("Preset", PresetType::Warm),
            drive: FloatParam::new(
                "Drive",
                5.0,
                FloatRange::Linear { min: 1.0, max: 10.0 },
            )
            .with_unit("dB")
            .with_smoother(SmoothingStyle::Logarithmic(50.0)),
            drywet: FloatParam::new(
                "Dry/Wet",
                1.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_unit("%")
            .with_value_to_string(formatters::v2s_f32_percentage(0))
            .with_smoother(SmoothingStyle::Linear(50.0)),
            riaa_intensity: FloatParam::new(
                "RIAA Intensity",
                0.5,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_unit("%")
            .with_value_to_string(formatters::v2s_f32_percentage(0))
            .with_smoother(SmoothingStyle::Linear(50.0)),
            parallel_mix: FloatParam::new(
                "Parallel Mix",
                0.5,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_unit("%")
            .with_value_to_string(formatters::v2s_f32_percentage(0))
            .with_smoother(SmoothingStyle::Linear(50.0)),
            auto_velocity: BoolParam::new("Auto Velocity", true),
        }
    }
}

impl Plugin for RustroverAiPlugin {
    const NAME: &'static str = "Rustrover AI - 2026 Patent";
    const VENDOR: &'static str = "Rustrover Audio";
    const URL: &'static str = "https://rustrover.audio";
    const EMAIL: &'static str = "info@rustrover.audio";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),
        ..AudioIOLayout::const_default()
    }];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        self.sample_rate = buffer_config.sample_rate;
        self.channels = array::from_fn(|_| ChannelDsp::new(self.sample_rate as u32));
        true
    }

    fn reset(&mut self) {
        for channel in &mut self.channels {
            channel.reset();
        }
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        // 특허 파라미터 읽기
        let _preset = self.params.preset.value();
        let drive = self.params.drive.smoothed.next();
        let drywet = self.params.drywet.smoothed.next();
        let riaa_intensity = self.params.riaa_intensity.smoothed.next();
        let parallel_mix = self.params.parallel_mix.smoothed.next();
        let auto_velocity = self.params.auto_velocity.value();

        let drive_linear = db_to_linear(drive);
        let saturation = ((drive - 1.0) / 9.0).clamp(0.0, 1.0);
        let magnetic = MagneticEQ::new(saturation, 0.5);

        for (channel_idx, mut channel_samples) in buffer.iter_samples().enumerate() {
            let velocity = if auto_velocity {
                VelocityAnalyzer::calculate_velocity(channel_samples)
            } else {
                0.6
            };

            let parallel_intensity = if velocity < 0.50 { parallel_mix } else { 0.0 };
            self.channels[channel_idx]
                .parallel
                .set_intensity(parallel_intensity);

            for sample in channel_samples.iter_mut() {
                let dry = *sample;
                let mut y = dry * drive_linear;
                y = soft_clip(y);

                if velocity < 0.60 {
                    y = magnetic.process(y);
                } else {
                    let riaa = self.channels[channel_idx].riaa.process(y);
                    y = mix_dry_wet(y, riaa, riaa_intensity);
                }

                y = self.channels[channel_idx].parallel.process(y);
                *sample = mix_dry_wet(dry, y, drywet);
            }
        }

        ProcessStatus::Normal
    }
}

#[inline]
fn db_to_linear(db: f32) -> f32 {
    10.0_f32.powf(db / 20.0)
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

impl ClapPlugin for RustroverAiPlugin {
    const CLAP_ID: &'static str = "com.rustrover.ai-patent-2026";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Velocity-adaptive audio processor with advanced feature analysis and sample-rate specific RIAA");
    const CLAP_MANUAL_URL: Option<&'static str> = Some("https://rustrover.audio/manual");
    const CLAP_SUPPORT_URL: Option<&'static str> = Some("https://rustrover.audio/support");
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Equalizer,
        ClapFeature::Compressor,
        ClapFeature::Distortion,
    ];
}

impl Vst3Plugin for RustroverAiPlugin {
    const VST3_CLASS_ID: [u8; 16] = *b"RustrvrPatent26\0";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Fx,
        Vst3SubCategory::Dynamics,
    ];
}

nih_export_clap!(RustroverAiPlugin);
nih_export_vst3!(RustroverAiPlugin);
