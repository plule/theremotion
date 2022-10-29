use std::fmt::Display;

use crossbeam_channel::{Receiver, Sender};

use egui::{FontFamily, FontId, Key, RichText, TextStyle};

use crate::{
    controls::{self},
    settings::Settings,
};

pub struct App {
    dsp_controls_rx: Receiver<controls::Controls>,
    controls: controls::Controls,
    settings: Settings,
    saved_settings: Settings,
    settings_tx: Sender<Settings>,
    main_tab: MainTab,
}

#[derive(Debug, PartialEq, Eq)]
pub enum MainTab {
    Play,
    RootEdit,
    ScaleEdit,
    Mix,
    Settings,
    Instructions,
}

impl Display for MainTab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MainTab::Play => f.write_str("Play"),
            MainTab::RootEdit => f.write_str("Root Edit"),
            MainTab::ScaleEdit => f.write_str("Scale Edit"),
            MainTab::Mix => f.write_str("Mix"),
            MainTab::Settings => f.write_str("Settings"),
            MainTab::Instructions => f.write_str("Instructions"),
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        dsp_controls_rx: Receiver<controls::Controls>,
        settings_tx: Sender<Settings>,
    ) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());

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
        } = self;

        if ctx.input().key_down(Key::Escape) {
            frame.close();
        }

        // Update the current control state from the DSP
        if let Some(new_controls) = dsp_controls_rx.try_iter().last() {
            *controls = new_controls;
        }

        egui::SidePanel::right("right_panel")
            .default_width(32.0)
            .show(ctx, |ui| {
                ui.vertical_centered_justified(|ui| {
                    ui.selectable_value(main_tab, MainTab::Play, RichText::new("👐").heading());
                    ui.selectable_value(main_tab, MainTab::RootEdit, RichText::new("🎵").heading());
                    ui.selectable_value(
                        main_tab,
                        MainTab::ScaleEdit,
                        RichText::new("🎼").heading(),
                    );
                    ui.selectable_value(main_tab, MainTab::Mix, RichText::new("🔊").heading());
                    ui.selectable_value(main_tab, MainTab::Settings, RichText::new("⛭").heading());
                    ui.selectable_value(
                        main_tab,
                        MainTab::Instructions,
                        RichText::new("ℹ").heading(),
                    );
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
                ui.heading(format!("{lh} {main_tab} {rh}"));
            });
            ui.separator();
            match main_tab {
                MainTab::Play => {
                    super::play_tab(ui, controls, settings);
                }
                MainTab::RootEdit => {
                    super::tab_root_edit(ui, controls, settings);
                }
                MainTab::ScaleEdit => {
                    super::tab_scale_edit(ui, controls, settings);
                }
                MainTab::Mix => {
                    super::tab_mix(ui, settings);
                }
                MainTab::Settings => {
                    super::tab_settings(ui, controls, settings);
                }
                MainTab::Instructions => {
                    super::tab_instructions(ui, controls, settings);
                }
            }
        });

        if saved_settings != settings {
            settings.save();
            *saved_settings = settings.clone();
            settings_tx.send(settings.clone()).unwrap();
        }
        ctx.request_repaint();
    }
}
