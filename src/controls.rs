use std::ops::RangeInclusive;

use anyhow::Result;
use crossbeam_channel::{SendError, Sender};
use faust_state::{Node, RangedInput, StateHandle, WidgetType};

use crate::{dsp_thread::ParameterUpdate, solfege::MidiNoteF};

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
    /// Drone detune
    pub drone_detune: Control,
    /// Drone "trumpet"
    pub drone_trumpet: Control,
    /// Drone notes
    pub drone_notes: [NoteControl; 4],
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
}

impl From<&StateHandle> for Controls {
    fn from(state: &StateHandle) -> Self {
        Self {
            lead: [0, 1, 2, 3].map(|i| {
                (
                    state.by_path(format!("lead/{i}/note").as_str()),
                    state.by_path(format!("lead/{i}/volume").as_str()),
                )
                    .into()
            }),
            lead_volume: state.by_path("lead/volume").into(),
            cutoff_note: state.by_path("filter/cutoffNote").into(),
            resonance: state.by_path("filter/res").into(),
            strum: [0, 1, 2, 3].map(|i| {
                (
                    state.by_path(format!("pluck/{i}/note").as_str()),
                    state.by_path(format!("pluck/{i}/gate").as_str()),
                )
                    .into()
            }),
            pluck_mute: state.by_path("pluck/mute").into(),
            drone_detune: state.by_path("drone/detune").into(),
            drone_trumpet: state.by_path("drone/trumpet").into(),
            drone_notes: [0, 1, 2, 3].map(|i| {
                (
                    state.by_path(format!("drone/{i}/note").as_str()),
                    state.by_path(format!("drone/{i}/volume").as_str()),
                )
                    .into()
            }),
            pitch_bend: state.by_path("pitchBend").into(),
            echo_mix: state.by_path("fx/echo/mix").into(),
            echo_duration: state.by_path("fx/echo/duration").into(),
            echo_feedback: state.by_path("fx/echo/feedback").into(),
            reverb_mix: state.by_path("fx/reverb/mix").into(),
            reverb_time: state.by_path("fx/reverb/time").into(),
            reverb_damp: state.by_path("fx/reverb/damp").into(),
            reverb_size: state.by_path("fx/reverb/size").into(),
            reverb_early_diff: state.by_path("fx/reverb/early_diff").into(),
            reverb_mod_depth: state.by_path("fx/reverb/mod_depth").into(),
            reverb_mod_freq: state.by_path("fx/reverb/mod_freq").into(),
            mix_master_volume: state.by_path("mix/master").into(),
            mix_drone_volume: state.by_path("mix/drone").into(),
            mix_lead_volume: state.by_path("mix/lead").into(),
            mix_pluck_volume: state.by_path("mix/pluck").into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Control {
    /// DSP metadata
    pub input: RangedInput,

    /// Parameter path in the DSP
    pub path: String,

    /// Parameter index in the DSP
    pub idx: i32,
}

impl Control {
    pub fn send(
        &self,
        dsp_tx: &Sender<ParameterUpdate>,
        value: f32,
    ) -> Result<(), SendError<ParameterUpdate>> {
        let range = &self.input.range;
        let value = value.clamp(*range.start(), *range.end());
        dsp_tx.send(ParameterUpdate::new(self.idx, value))
    }

    pub fn get_scaled(&self, value: f32, value_range: &RangeInclusive<f32>) -> f32 {
        convert_range(value, value_range, &self.input.range)
            .clamp(*self.input.range.start(), *self.input.range.end())
    }
}

impl From<NodeIndex<'_>> for Control {
    fn from(nodeindex: NodeIndex<'_>) -> Self {
        let input = match nodeindex.1.widget_type() {
            WidgetType::VerticalSlider(input) => input,
            WidgetType::HorizontalSlider(input) => input,
            WidgetType::NumEntry(input) => input,
            _ => panic!(
                "The parameter {} is not a ranged input.",
                nodeindex.1.path()
            ),
        };
        Self {
            input: input.clone(),
            path: nodeindex.1.path(),
            idx: nodeindex.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BoolControl {
    /// Idx for the DSP
    pub idx: i32,

    /// Path for the dsp
    pub path: String,
}

impl BoolControl {
    pub fn send(&self, tx: &Sender<ParameterUpdate>, value: bool) {
        tx.send(ParameterUpdate::new(
            self.idx,
            if value { 1.0 } else { 0.0 },
        ))
        .unwrap();
    }
}

impl From<NodeIndex<'_>> for BoolControl {
    fn from(nodeindex: NodeIndex<'_>) -> Self {
        Self {
            idx: nodeindex.0,
            path: nodeindex.1.path(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct NoteControl {
    /// Control for the pitch of the note
    pub note: Control,

    /// Control for the volume of the note
    pub volume: Control,
}

impl NoteControl {
    pub fn send_note(
        &self,
        dsp_tx: &Sender<ParameterUpdate>,
        note: &MidiNoteF,
    ) -> Result<(), SendError<ParameterUpdate>> {
        self.note.send(dsp_tx, note.note())
    }
}

impl From<(NodeIndex<'_>, NodeIndex<'_>)> for NoteControl {
    fn from((note, volume): (NodeIndex<'_>, NodeIndex<'_>)) -> Self {
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

impl PluckControl {
    pub fn send_note(
        &self,
        dsp_tx: &Sender<ParameterUpdate>,
        note: &MidiNoteF,
    ) -> Result<(), SendError<ParameterUpdate>> {
        self.note.send(dsp_tx, note.note())
    }
}

impl From<(NodeIndex<'_>, NodeIndex<'_>)> for PluckControl {
    fn from((note, pluck): (NodeIndex<'_>, NodeIndex<'_>)) -> Self {
        Self {
            note: note.into(),
            pluck: pluck.into(),
        }
    }
}

pub struct NodeIndex<'a>(i32, &'a Node);

trait NodeByPath {
    fn by_path(&self, path: &str) -> NodeIndex<'_>;
}

impl NodeByPath for StateHandle {
    fn by_path(&self, path: &str) -> NodeIndex<'_> {
        for n in self.params().iter() {
            if n.1.path() == path {
                return NodeIndex(*n.0, n.1);
            }
        }
        panic!("Wrongly parameterized parameter {path}");
    }
}

pub fn convert_range(
    value: f32,
    input_range: &RangeInclusive<f32>,
    output_range: &RangeInclusive<f32>,
) -> f32 {
    let in_min = *input_range.start();
    let in_max = *input_range.end();
    let out_min = *output_range.start();
    let out_max = *output_range.end();
    (((value - in_min) * (out_max - out_min)) / (in_max - in_min)) + out_min
}
