use std::ops::RangeInclusive;

use faust_state::{Node, RangedInput, StateHandle, WidgetType};

use crate::settings::Preset;

/// Ability to exchange with the DSP state
pub trait ControlTrait {
    /// Set the current state to the DSP
    fn send(&mut self, state: &mut StateHandle);
}

/// DSP controls
#[derive(Debug, Clone)]
pub struct Controls {
    /// Lead voice chord
    pub lead: [NoteControl; 4],
    /// Global lead volume
    pub lead_volume: Control,
    /// Filter cutoff
    pub cutoff_note: Control,
    /// Filter resonance
    pub resonance: Control,
    /// Guitar strum
    pub strum: [PluckControl; 4],
    /// Guitar pluck damping
    pub pluck_mute: Control,
    /// Drone volume
    pub drone_volume: Control,
    /// Drone note
    pub drone_note: Control,
    /// Global pitch bend (guitar+lead)
    pub pitch_bend: Control,

    /// Echo
    pub echo_mix: Control,
    pub echo_duration: Control,
    pub echo_feedback: Control,

    /// Reverb
    pub reverb_mix: Control,
    pub reverb_time: Control,
    pub reverb_damp: Control,
    pub reverb_size: Control,
    pub reverb_early_diff: Control,
    pub reverb_mod_depth: Control,
    pub reverb_mod_freq: Control,

    /// Mix
    pub mix_master_volume: Control,
    pub mix_drone_volume: Control,
    pub mix_lead_volume: Control,
    pub mix_pluck_volume: Control,

    /// Raw note for the UI
    pub raw_note: f32,

    /// Autotune amount for the UI
    pub autotune: usize,

    /// Warning message
    pub warning: Option<String>,
    /// Error message
    pub error: Option<String>,

    pub has_hands: (bool, bool),
}

impl Controls {
    pub fn update_from_preset(&mut self, preset: &Preset) {
        self.mix_master_volume.value = preset.mix.master;
        self.mix_lead_volume.value = preset.mix.lead;
        self.mix_pluck_volume.value = preset.mix.guitar;
        self.mix_drone_volume.value = preset.mix.drone;
        self.echo_duration.value = preset.fx.echo.duration;
        self.echo_feedback.value = preset.fx.echo.feedback;
        self.echo_mix.value = preset.fx.echo.mix;
        self.reverb_mix.value = preset.fx.reverb.mix;
        self.reverb_time.value = preset.fx.reverb.time;
        self.reverb_damp.value = preset.fx.reverb.damp;
        self.reverb_size.value = preset.fx.reverb.size;
    }
}

impl ControlTrait for Controls {
    fn send(&mut self, state: &mut StateHandle) {
        for note in &mut self.lead {
            note.send(state);
        }
        self.lead_volume.send(state);
        self.cutoff_note.send(state);
        self.resonance.send(state);
        for string in &mut self.strum {
            string.send(state);
        }
        self.pluck_mute.send(state);
        self.drone_volume.send(state);
        self.drone_note.send(state);
        self.pitch_bend.send(state);
        self.mix_master_volume.send(state);
        self.mix_lead_volume.send(state);
        self.mix_pluck_volume.send(state);
        self.mix_drone_volume.send(state);
        self.echo_mix.send(state);
        self.echo_feedback.send(state);
        self.echo_duration.send(state);
        self.reverb_mix.send(state);
        self.reverb_time.send(state);
        self.reverb_damp.send(state);
        self.reverb_size.send(state);
        self.reverb_early_diff.send(state);
        self.reverb_mod_depth.send(state);
        self.reverb_mod_freq.send(state);
        state.send();
    }
}

impl From<&StateHandle> for Controls {
    fn from(state: &StateHandle) -> Self {
        Self {
            lead: [0, 1, 2, 3].map(|i| {
                (
                    state
                        .node_by_path(format!("lead/{}/note", i).as_str())
                        .unwrap(),
                    state
                        .node_by_path(format!("lead/{}/volume", i).as_str())
                        .unwrap(),
                )
                    .into()
            }),
            lead_volume: state.node_by_path("lead/volume").unwrap().into(),
            cutoff_note: state.node_by_path("lead/cutoffNote").unwrap().into(),
            resonance: state.node_by_path("lead/res").unwrap().into(),
            strum: [0, 1, 2, 3].map(|i| {
                (
                    state
                        .node_by_path(format!("pluck/{}/note", i).as_str())
                        .unwrap(),
                    state
                        .node_by_path(format!("pluck/{}/gate", i).as_str())
                        .unwrap(),
                )
                    .into()
            }),
            pluck_mute: state.node_by_path("pluck/mute").unwrap().into(),
            drone_volume: state.node_by_path("drone/volume").unwrap().into(),
            drone_note: state.node_by_path("drone/note").unwrap().into(),
            pitch_bend: state.node_by_path("pluck/pitchBend").unwrap().into(),
            echo_mix: state.node_by_path("fx/echo/mix").unwrap().into(),
            echo_duration: state.node_by_path("fx/echo/duration").unwrap().into(),
            echo_feedback: state.node_by_path("fx/echo/feedback").unwrap().into(),
            reverb_mix: state.node_by_path("fx/reverb/mix").unwrap().into(),
            reverb_time: state.node_by_path("fx/reverb/time").unwrap().into(),
            reverb_damp: state.node_by_path("fx/reverb/damp").unwrap().into(),
            reverb_size: state.node_by_path("fx/reverb/size").unwrap().into(),
            reverb_early_diff: state.node_by_path("fx/reverb/early_diff").unwrap().into(),
            reverb_mod_depth: state.node_by_path("fx/reverb/mod_depth").unwrap().into(),
            reverb_mod_freq: state.node_by_path("fx/reverb/mod_freq").unwrap().into(),
            mix_master_volume: state.node_by_path("mix/master").unwrap().into(),
            mix_drone_volume: state.node_by_path("mix/drone").unwrap().into(),
            mix_lead_volume: state.node_by_path("mix/lead").unwrap().into(),
            mix_pluck_volume: state.node_by_path("mix/pluck").unwrap().into(),
            raw_note: 0.0,
            autotune: 0,
            warning: None,
            error: None,
            has_hands: (false, false),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Control {
    /// Current value of the control in the DSP
    pub value: f32,

    /// DSP metadata
    pub input: RangedInput,

    /// Name for the DSP
    pub path: String,
}

impl ControlTrait for Control {
    fn send(&mut self, state: &mut StateHandle) {
        state.set_by_path(&self.path, self.value).unwrap();
    }
}

impl Control {
    pub fn set_scaled(&mut self, value: f32, value_range: RangeInclusive<f32>) {
        self.value = convert_range(value, value_range, &self.input.range);
    }
}

impl From<&Node> for Control {
    fn from(node: &Node) -> Self {
        let input = match node.widget_type() {
            WidgetType::VerticalSlider(input) => input,
            WidgetType::HorizontalSlider(input) => input,
            WidgetType::NumEntry(input) => input,
            _ => panic!("The parameter {} is not a ranged input.", node.path()),
        };
        Self {
            value: input.init,
            input: input.clone(),
            path: node.path(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BoolControl {
    /// On-off state
    pub value: bool,

    /// Name for the DSP
    pub path: String,
}

impl ControlTrait for BoolControl {
    fn send(&mut self, state: &mut StateHandle) {
        state
            .set_by_path(&self.path, if self.value { 1.0 } else { 0.0 })
            .unwrap();
    }
}

impl From<&Node> for BoolControl {
    fn from(node: &Node) -> Self {
        let value = node.widget_type().init_value() > 0.5;
        let path = node.path();
        Self { value, path }
    }
}

#[derive(Debug, Clone)]
pub struct NoteControl {
    /// Control for the pitch of the note
    pub note: Control,

    /// Control for the volume of the note
    pub volume: Control,
}

impl ControlTrait for NoteControl {
    fn send(&mut self, state: &mut StateHandle) {
        self.note.send(state);
        self.volume.send(state);
    }
}
impl From<(&Node, &Node)> for NoteControl {
    fn from((note, volume): (&Node, &Node)) -> Self {
        Self {
            note: note.into(),
            volume: volume.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PluckControl {
    /// Control for the pitch of the note
    pub note: Control,

    /// Control for the pluck impulse
    pub pluck: BoolControl,
}

impl ControlTrait for PluckControl {
    fn send(&mut self, state: &mut StateHandle) {
        self.note.send(state);
        self.pluck.send(state);
    }
}
impl From<(&Node, &Node)> for PluckControl {
    fn from((note, pluck): (&Node, &Node)) -> Self {
        Self {
            note: note.into(),
            pluck: pluck.into(),
        }
    }
}

trait NodeByPath {
    fn node_by_path(&self, path: &str) -> Option<&Node>;
}

impl NodeByPath for StateHandle {
    fn node_by_path(&self, path: &str) -> Option<&Node> {
        self.params().values().find(|n| n.path() == path)
    }
}

pub fn convert_range(
    value: f32,
    input_range: RangeInclusive<f32>,
    output_range: &RangeInclusive<f32>,
) -> f32 {
    {
        let in_min = *input_range.start();
        let in_max = *input_range.end();
        let out_min = *output_range.start();
        let out_max = *output_range.end();
        ((((value - in_min) * (out_max - out_min)) / (in_max - in_min)) + out_min)
            .clamp(out_min, out_max)
    }
}
