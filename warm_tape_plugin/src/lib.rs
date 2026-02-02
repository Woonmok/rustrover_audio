use nih_plug::prelude::*;
use dsp_core::{process_block, DspParams};
use std::sync::Arc;

pub struct WarmTapePlugin {
    params: Arc<WarmTapeParams>,
    sample_rate: f32,
}

#[derive(Params)]
pub struct WarmTapeParams {
    #[id = "drive"]
    pub drive: FloatParam,
    
    #[id = "warmth"]
    pub warmth: FloatParam,
}

impl Default for WarmTapePlugin {
    fn default() -> Self {
        Self {
            params: Arc::new(WarmTapeParams::default()),
            sample_rate: 48000.0,
        }
    }
}

impl Default for WarmTapeParams {
    fn default() -> Self {
        Self {
            drive: FloatParam::new(
                "Drive",
                0.0,
                FloatRange::Linear { min: 0.0, max: 20.0 },
            )
            .with_unit(" dB")
            .with_smoother(SmoothingStyle::Linear(50.0)),
            
            warmth: FloatParam::new(
                "Warmth",
                0.5,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_smoother(SmoothingStyle::Linear(50.0)),
        }
    }
}

impl Plugin for WarmTapePlugin {
    const NAME: &'static str = "Warm Tape";
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
        true
    }

    fn reset(&mut self) {}

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let drive = self.params.drive.smoothed.next();
        let sample_rate = self.sample_rate;
        
        let params = DspParams { drive_db: drive };

        for mut channel_samples in buffer.iter_samples() {
            let params = DspParams { drive_db: drive };
            
            for sample in channel_samples.iter_mut() {
                let x = *sample * db_to_linear(drive);
                let y = soft_clip(x);
                *sample = y;
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

impl ClapPlugin for WarmTapePlugin {
    const CLAP_ID: &'static str = "com.rustrover.warm-tape";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Warm tape saturation plugin");
    const CLAP_MANUAL_URL: Option<&'static str> = None;
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Distortion,
    ];
}

impl Vst3Plugin for WarmTapePlugin {
    const VST3_CLASS_ID: [u8; 16] = *b"RustrvrWarmTape\0";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Fx,
        Vst3SubCategory::Distortion,
    ];
}

nih_export_clap!(WarmTapePlugin);
nih_export_vst3!(WarmTapePlugin);
