use egui::RichText;
use staff::Pitch;

use crate::{controls, settings::Settings};

use super::KeyboardEditMode;

pub fn tab_root_edit(
    ui: &mut egui::Ui,
    controls: &mut controls::Controls,
    settings: &mut Settings,
) {
    ui.add(crate::ui::Keyboard::new(
        controls.lead.iter().collect(),
        settings,
        KeyboardEditMode::RootNote,
    ));
    ui.separator();
    octave_selector(ui, "Lead Octave", &mut settings.octave);
    octave_selector(ui, "Guitar Octave", &mut settings.guitar_octave);
    ui.separator();
    ui.vertical_centered_justified(|ui| {
        ui.label(RichText::new("Note").size(30.0));
    });
    ui.horizontal_wrapped(|ui| {
        for pitch in 0..=11 {
            let pitch = Pitch::from_byte(pitch);
            ui.selectable_value(
                &mut settings.pitch,
                pitch,
                RichText::new(format!("  {}  ", pitch)).size(40.0),
            );
        }
    });
}

fn octave_selector(ui: &mut egui::Ui, name: &str, octave_value: &mut i8) {
    ui.horizontal_wrapped(|ui| {
        ui.label(RichText::new(name).size(40.0));
        for octave in 0..=4 {
            ui.selectable_value(
                octave_value,
                octave,
                RichText::new(format!("  {}  ", octave)).size(40.0),
            );
        }
    });
}
