use strum::{EnumIter, IntoEnumIterator};

use crossbeam_channel::{Receiver, Sender};

use egui::{FontFamily, FontId, Key, RichText, TextStyle};

use crate::{controls, settings::Settings};

pub struct App {
    dsp_controls_rx: Receiver<controls::Controls>,
    controls: controls::Controls,
    settings: Settings,
    saved_settings: Settings,
    settings_tx: Sender<Settings>,
    main_tab: MainTab,
    tabtip: bool,
}

#[derive(Debug, PartialEq, Eq, EnumIter, Clone, Copy)]
pub enum MainTab {
    Play,
    RootEdit,
    Scale,
    Mix,
    Effects,
    Presets,
    Instructions,
}

impl MainTab {
    pub fn title(&self) -> &str {
        match self {
            MainTab::Play => "Play",
            MainTab::RootEdit => "Root Note",
            MainTab::Scale => "Scale",
            MainTab::Mix => "Mix",
            MainTab::Effects => "Effects",
            MainTab::Presets => "Presets",
            MainTab::Instructions => "Instructions",
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            MainTab::Play => "👐",
            MainTab::RootEdit => "🎵",
            MainTab::Scale => "🎼",
            MainTab::Mix => "🎚️",
            MainTab::Effects => "🎛️",
            MainTab::Presets => "💾",
            MainTab::Instructions => "ℹ",
        }
    }

    pub fn add_widget<'a>(
        &self,
        ui: &mut egui::Ui,
        controls: &'a mut controls::Controls,
        settings: &'a mut Settings,
        tabtip: bool,
    ) {
        match self {
            MainTab::Play => ui.add(super::TabPlay::new(controls, &mut settings.current_preset)),
            MainTab::RootEdit => ui.add(super::TabRootNote::new(
                controls,
                &mut settings.current_preset,
            )),
            MainTab::Scale => ui.add(super::TabScale::new(controls, &mut settings.current_preset)),
            MainTab::Mix => ui.add(super::TabMix::new(&mut settings.current_preset)),
            MainTab::Effects => ui.add(super::TabEffects::new(
                controls,
                &mut settings.current_preset,
            )),
            MainTab::Presets => ui.add(super::TabPresets::new(settings, tabtip)),
            MainTab::Instructions => ui.add(super::TabInstructions::new()),
        };
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        dsp_controls_rx: Receiver<controls::Controls>,
        settings_tx: Sender<Settings>,
        tabtip: bool,
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
                FontId::new(40.0, FontFamily::Proportional),
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
        cc.egui_ctx.set_style(style);

        let controls = dsp_controls_rx.recv().unwrap();
        let settings = Settings::read();
        settings_tx.send(settings.clone()).unwrap();
        Self {
            dsp_controls_rx,
            settings_tx,
            controls,
            saved_settings: settings.clone(),
            main_tab: MainTab::Play,
            settings,
            tabtip,
        }
    }
}

impl eframe::App for App {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self {
            dsp_controls_rx,
            controls,
            settings,
            settings_tx,
            saved_settings,
            main_tab,
            tabtip: _,
        } = self;

        if ctx.input().key_down(Key::Escape) {
            frame.close();
        }

        // Update the current control state from the DSP
        if let Some(new_controls) = dsp_controls_rx.try_iter().last() {
            *controls = new_controls;
        }

        egui::SidePanel::right("right_panel")
            .default_width(24.0)
            .show(ctx, |ui| {
                ui.vertical_centered_justified(|ui| {
                    for tab in MainTab::iter() {
                        ui.selectable_value(
                            main_tab,
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
                if let Some(warning) = &controls.warning {
                    let warning = format!("⚠ Leap: {}", warning);
                    ui.label(RichText::new(warning).color(egui::Color32::YELLOW));
                }

                if let Some(error) = &controls.error {
                    let error = format!("⚠ Leap: {}", error);
                    ui.label(RichText::new(error).color(egui::Color32::RED));
                }

                egui::warn_if_debug_build(ui);
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                let (lh, rh) = controls.has_hands;
                let lh = if lh { "👈" } else { "⬜" };
                let rh = if rh { "👉" } else { "⬜" };
                let title = main_tab.title();
                ui.heading(format!("{lh} {title} {rh}"));
            });
            ui.separator();
            main_tab.add_widget(ui, controls, settings, self.tabtip);
        });

        if saved_settings != settings {
            settings.save();
            *saved_settings = settings.clone();
            settings_tx.send(settings.clone()).unwrap();
        }
        ctx.request_repaint();
    }
}
