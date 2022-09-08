use std::ops::RangeInclusive;

use faust_state::{Node, RangedInput, StateHandle, WidgetType};

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
    /// Filter resonnance
    pub resonance: Control,
    /// Supersaw volume
    pub supersaw: Control,
    /// Supersaw detune
    pub detune: Control,
    /// Guitar pluck
    pub pluck: BoolControl,
    /// Guitare note
    pub pluck_note: Control,
    /// Guitar pluck gain
    pub pluck_gain: Control,
    /// Guitar pluck damping
    pub pluck_damping: Control,
    /// Drone volume
    pub drone_volume: Control,
    /// Drone note
    pub drone_note: Control,

    /// Raw note for the UI
    pub raw_note: f32,

    /// Autotune amount for the UI
    pub autotune: usize,

    /// Warning message
    pub warning: Option<String>,
    /// Error message
    pub error: Option<String>,
}

impl ControlTrait for Controls {
    fn send(&mut self, state: &mut StateHandle) {
        for note in &mut self.lead {
            note.send(state);
        }
        self.lead_volume.send(state);
        self.cutoff_note.send(state);
        self.resonance.send(state);
        self.supersaw.send(state);
        self.detune.send(state);
        self.pluck.send(state);
        self.pluck_note.send(state);
        self.pluck_gain.send(state);
        self.pluck_damping.send(state);
        self.drone_volume.send(state);
        self.drone_note.send(state);
        state.send();
    }
}

impl From<&StateHandle> for Controls {
    fn from(state: &StateHandle) -> Self {
        Self {
            lead: [1, 2, 3, 4].map(|i| {
                (
                    state
                        .node_by_path(format!("lead/chord/note{}", i).as_str())
                        .unwrap(),
                    state
                        .node_by_path(format!("lead/chord/vol{}", i).as_str())
                        .unwrap(),
                )
                    .into()
            }),
            lead_volume: state.node_by_path("lead/vol").unwrap().into(),
            cutoff_note: state
                .node_by_path("lead/filter/cutoff_note")
                .unwrap()
                .into(),
            resonance: state.node_by_path("lead/filter/res").unwrap().into(),
            supersaw: state.node_by_path("lead/supersaw/volume").unwrap().into(),
            detune: state.node_by_path("lead/supersaw/detune").unwrap().into(),
            pluck: state.node_by_path("pluck/gate").unwrap().into(),
            pluck_note: state.node_by_path("pluck/note").unwrap().into(),
            pluck_gain: state.node_by_path("pluck/gain").unwrap().into(),
            pluck_damping: state.node_by_path("pluck/damping").unwrap().into(),
            drone_volume: state.node_by_path("drone/volume").unwrap().into(),
            drone_note: state.node_by_path("drone/note").unwrap().into(),
            raw_note: 0.0,
            autotune: 0,
            warning: None,
            error: None,
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
