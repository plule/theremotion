use staff::scale::ScaleIntervals;

use crate::{controls, scales::MoreScales, settings::Settings};

use super::KeyboardEditMode;

pub fn tab_scale_edit(
    ui: &mut egui::Ui,
    controls: &mut controls::Controls,
    settings: &mut Settings,
) {
    ui.add(crate::ui::Keyboard::new(
        controls.lead.iter().collect(),
        settings,
        KeyboardEditMode::Scale,
    ));
    ui.separator();
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().button_padding.x = 10.0;
        ui.spacing_mut().button_padding.y = 10.0;
        ui.selectable_value(&mut settings.scale, ScaleIntervals::all(), "🎼 Chromatic");
        ui.selectable_value(&mut settings.scale, ScaleIntervals::major(), "🎼 Major");
        ui.selectable_value(
            &mut settings.scale,
            ScaleIntervals::melodic_minor(),
            "🎼 Melodic Minor",
        );
        ui.selectable_value(
            &mut settings.scale,
            ScaleIntervals::harmonic_minor(),
            "🎼 Harmonic Minor",
        );
        ui.selectable_value(
            &mut settings.scale,
            ScaleIntervals::natural_minor(),
            "🎼 Natural Minor",
        );
        ui.selectable_value(&mut settings.scale, ScaleIntervals::dorian(), "🎼 Dorian");
        ui.selectable_value(&mut settings.scale, ScaleIntervals::blues(), "🎼 Blues");
        ui.selectable_value(
            &mut settings.scale,
            ScaleIntervals::freygish(),
            "🎼 Freygish",
        );
        ui.selectable_value(
            &mut settings.scale,
            ScaleIntervals::altered_dorian(),
            "🎼 Altered Dorian",
        );
    });
}
