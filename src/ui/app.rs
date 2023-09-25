use strum::{EnumIter, IntoEnumIterator};

use crossbeam_channel::{Receiver, Sender};

use egui::{FontFamily, FontId, Key, RichText, TextStyle};

use crate::{
    controls::Controls, dsp_thread::ParameterUpdate, settings::Settings, solfege::MidiNoteF,
};

use super::UiUpdate;

/// Application state
pub struct App {
    update_rx: Receiver<UiUpdate>,
    controls: Controls,
    settings: Settings,
    saved_settings: Settings,
    settings_tx: Sender<Settings>,
    dsp_tx: Sender<ParameterUpdate>,
    main_tab: MainTab,
    error: Option<String>,
    lead_volume: f32,
    lead_chord_notes: [MidiNoteF; 4],
    lead_chord_volumes: [f32; 4],
    chords_number: f32,
    raw_note: MidiNoteF,
    filter_cutoff: f32,
    filter_resonance: f32,
    autotune_amount: usize,
    has_hands: (bool, bool),
    pitch_xy: (f32, f32),
    current_scale_name: String,
    strum_ready: bool,
    trumpet_strength: f32,
}

/// List of tabs in the application
#[derive(Debug, PartialEq, Eq, EnumIter, Clone, Copy)]
pub enum MainTab {
    /// Play screen
    Play,
    /// Root note editor
    RootEdit,
    /// Scale editor
    Scale,
    /// Mix table
    Mix,
    /// Effects control
    Effects,
    /// Preset management
    Presets,
    /// Application settings
    Settings,
}

impl MainTab {
    /// Title of the tab
    pub fn title(&self) -> &str {
        match self {
            MainTab::Play => "Play",
            MainTab::RootEdit => "Root Note",
            MainTab::Scale => "Scale",
            MainTab::Mix => "Mix",
            MainTab::Effects => "Effects",
            MainTab::Presets => "Presets",
            MainTab::Settings => "Settings",
        }
    }

    /// Emoji icon of the tab
    pub fn icon(&self) -> &str {
        match self {
            MainTab::Play => "👐",
            MainTab::RootEdit => "🎵",
            MainTab::Scale => "🎼",
            MainTab::Mix => "🎚️",
            MainTab::Effects => "🎛️",
            MainTab::Presets => "💾",
            MainTab::Settings => "⚙️",
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        update_rx: Receiver<UiUpdate>,
        dsp_tx: Sender<ParameterUpdate>,
        settings_tx: Sender<Settings>,
        controls: Controls,
        settings: Settings,
    ) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "noto_emoji".to_owned(),
            egui::FontData::from_static(include_bytes!("NotoEmoji-Bold.ttf")),
        );

        fonts
            .families
            .entry(egui::FontFamily::Name("icons".into()))
            .or_default()
            .insert(0, "noto_emoji".to_owned());
        cc.egui_ctx.set_fonts(fonts);

        let mut style = (*cc.egui_ctx.style()).clone();
        style.text_styles = [
            (
                TextStyle::Small,
                FontId::new(15.0, FontFamily::Proportional),
            ),
            (TextStyle::Body, FontId::new(21.0, FontFamily::Proportional)),
            (
                TextStyle::Button,
                FontId::new(35.0, FontFamily::Proportional),
            ),
            (
                TextStyle::Heading,
                FontId::new(64.0, FontFamily::Proportional),
            ),
            (
                TextStyle::Monospace,
                FontId::new(21.0, FontFamily::Monospace),
            ),
        ]
        .into();
        style.spacing.interact_size = egui::vec2(64.0, 64.0);
        style.spacing.slider_width = 280.0;
        style.spacing.button_padding.x = 10.0;
        style.spacing.button_padding.y = 10.0;
        cc.egui_ctx.set_style(style);

        Self {
            update_rx,
            dsp_tx,
            settings_tx,
            saved_settings: settings.clone(),
            main_tab: MainTab::Play,
            settings,
            error: None,
            lead_volume: controls.lead_volume.input.init,
            lead_chord_notes: controls
                .lead
                .clone()
                .map(|n| MidiNoteF::new(n.note.input.init)),
            lead_chord_volumes: controls.lead.clone().map(|n| n.volume.input.init),
            chords_number: 0.0,
            raw_note: MidiNoteF::new(controls.lead[0].note.input.init),
            filter_cutoff: controls.cutoff_note.input.init,
            filter_resonance: controls.resonance.input.init,
            autotune_amount: 0,
            has_hands: (false, false),
            pitch_xy: (0.0, 0.0),
            controls,
            current_scale_name: String::default(),
            strum_ready: false,
            trumpet_strength: 0.0,
        }
    }

    fn draw_current_tab(&mut self, ui: &mut egui::Ui) {
        match self.main_tab {
            MainTab::Play => ui.add(super::TabPlay {
                controls: &self.controls,
                settings: &mut self.settings,
                lead_volume: self.lead_volume,
                lead_chord_notes: &self.lead_chord_notes,
                lead_chord_volumes: &self.lead_chord_volumes,
                raw_note: self.raw_note,
                filter_cutoff: self.filter_cutoff,
                filter_resonance: self.filter_resonance,
                pitch_xy: self.pitch_xy,
                chords_number: self.chords_number,
                autotune_amount: (self.autotune_amount as f32) / 5.0,
                strum_ready: self.strum_ready,
                trumpet_strength: self.trumpet_strength,
            }),
            MainTab::RootEdit => ui.add(super::TabRootNote::new(
                &mut self.settings.current_preset,
                &self.lead_chord_notes,
                &self.lead_chord_volumes,
            )),
            MainTab::Scale => ui.add(super::TabScale::new(
                &mut self.settings,
                &self.lead_chord_notes,
                &self.lead_chord_volumes,
                &mut self.current_scale_name,
            )),
            MainTab::Mix => ui.add(super::TabMix::new(&mut self.settings.current_preset)),
            MainTab::Effects => ui.add(super::TabEffects::new(
                &self.controls,
                &mut self.settings.current_preset,
            )),
            MainTab::Presets => ui.add(super::TabPresets::new(&mut self.settings)),
            MainTab::Settings => ui.add(super::TabSettings::new(&mut self.settings)),
        };
    }

    fn receive_update(&mut self) {
        for msg in self.update_rx.try_iter() {
            match msg {
                UiUpdate::Error(x) => self.error = Some(x),
                UiUpdate::ErrorReset => self.error = None,
                UiUpdate::LeadVolume(x) => self.lead_volume = x,
                UiUpdate::LeadChordNotes(x) => self.lead_chord_notes = x,
                UiUpdate::LeadChordVolumes(x) => self.lead_chord_volumes = x,
                UiUpdate::ChordsNumber(x) => self.chords_number = x,
                UiUpdate::RawNote(x) => self.raw_note = x,
                UiUpdate::Filter(cutoff, resonance) => {
                    self.filter_cutoff = cutoff;
                    self.filter_resonance = resonance;
                }
                UiUpdate::AutotuneAmount(x) => self.autotune_amount = x,
                UiUpdate::HasHands(left, right) => self.has_hands = (left, right),
                UiUpdate::PitchXY(x, y) => self.pitch_xy = (x, y),
                UiUpdate::StrumReady(x) => self.strum_ready = x,
                UiUpdate::TrumpetStrength(x) => self.trumpet_strength = x,
                UiUpdate::Settings(settings) => self.settings.current_preset = settings,
            }
        }
    }
}

impl eframe::App for App {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.input(|i| {
            if i.key_down(Key::Escape) {
                frame.close();
            }
        });

        // Receive UI update messages
        self.receive_update();

        egui::SidePanel::right("right_panel")
            .default_width(24.0)
            .show(ctx, |ui| {
                ui.vertical_centered_justified(|ui| {
                    for tab in MainTab::iter() {
                        ui.selectable_value(
                            &mut self.main_tab,
                            tab,
                            RichText::new(tab.icon())
                                .text_style(egui::TextStyle::Button)
                                .family(egui::FontFamily::Name("icons".into())),
                        );
                    }
                });
            });
        egui::TopBottomPanel::bottom("bottom_panel")
            .default_height(32.0)
            .show(ctx, |ui| {
                if let Some(error) = &self.error {
                    let error = format!("⚠ Leap: {error}");
                    ui.label(RichText::new(error).color(egui::Color32::RED));
                }

                egui::warn_if_debug_build(ui);
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                let (lh, rh) = self.has_hands;
                let lh = if lh { "👈" } else { "⬜" };
                let rh = if rh { "👉" } else { "⬜" };
                let title = self.main_tab.title();
                ui.heading(format!("{lh} {title} {rh}"));
            });
            ui.separator();
            self.draw_current_tab(ui);
        });

        if self.saved_settings != self.settings {
            if let Err(err) = self.settings.save() {
                log::error!("{}", err);
            }
            self.saved_settings = self.settings.clone();
            self.settings_tx.send(self.settings.clone()).unwrap();
            self.settings
                .current_preset
                .send_to_dsp(&self.controls, &self.dsp_tx)
                .unwrap();
        }
        ctx.request_repaint();
    }
}
