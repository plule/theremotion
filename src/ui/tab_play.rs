use std::ops::RangeInclusive;

use egui::plot::{
    uniform_grid_spacer, GridMark, HLine, Legend, Line, MarkerShape, PlotPoint, PlotPoints, Points,
    VLine,
};
use staff::midi::MidiNote;

use crate::{
    controls::{self},
    settings::Settings,
    ui::KeyboardEditMode,
};

pub fn play_tab(ui: &mut egui::Ui, controls: &mut controls::Controls, settings: &mut Settings) {
    ui.add(crate::ui::Keyboard::new(
        controls.lead.iter().collect(),
        settings,
        KeyboardEditMode::Drone,
    ));
    ui.separator();
    ui.horizontal(|ui| {
        autotune_plot(
            ui,
            250.0,
            settings,
            controls.autotune,
            controls.raw_note,
            controls.lead[0].note.value,
        );

        ui.add_space(10.0);

        tuner(
            ui,
            100.0,
            250.0,
            settings,
            "tuner",
            controls.raw_note,
            controls.lead[0].note.value,
        );

        ui.add_space(10.0);

        xy_plot(
            ui,
            250.0,
            "rh_hand",
            &controls.cutoff_note,
            &controls.resonance,
            "Cutoff",
            "Resonance",
        );
    });
}

fn autotune_plot(
    ui: &mut egui::Ui,
    size: f32,
    settings: &mut Settings,
    autotune: usize,
    raw_value: f32,
    value: f32,
) {
    let note_range = settings.note_range();
    let smooths = (*note_range.start() as usize * 10..*note_range.end() as usize * 10).map(|i| {
        let x = i as f32 * 0.1;
        PlotPoint::new(
            x,
            crate::music_theory::autotune(x, autotune, settings.scale_notes()),
        )
    });
    let line = Line::new(PlotPoints::Owned(smooths.collect()));
    // hack: force the include_x/include_y to recenter on root note change
    let plot_id = format!("{}{}", settings.pitch, settings.octave);
    egui::plot::Plot::new(plot_id)
        .allow_boxed_zoom(false)
        .allow_drag(false)
        .allow_scroll(false)
        .allow_zoom(false)
        .include_x(*note_range.start())
        .include_x(*note_range.end())
        .include_y(*note_range.start())
        .include_y(*note_range.end())
        .x_grid_spacer(uniform_grid_spacer(|_| [12.0, 1.0, 1.0]))
        .y_grid_spacer(uniform_grid_spacer(|_| [12.0, 1.0, 1.0]))
        .x_axis_formatter(note_formatter)
        .y_axis_formatter(note_formatter)
        .legend(Legend::default())
        .width(size)
        .height(size)
        .show(ui, |plot_ui| {
            plot_ui.line(line);
            plot_ui.points(
                Points::new(PlotPoints::Owned(vec![PlotPoint::new(raw_value, value)]))
                    .shape(MarkerShape::Plus)
                    .radius(6.0),
            );
        });
}

fn note_formatter(note: f64, _range: &RangeInclusive<f64>) -> String {
    MidiNote::from_byte(note as u8).to_string()
}

fn xy_plot(
    ui: &mut egui::Ui,
    size: f32,
    plot_name: &str,
    control_x: &controls::Control,
    control_y: &controls::Control,
    control_x_name: &str,
    control_y_name: &str,
) {
    egui::plot::Plot::new(plot_name)
        .allow_boxed_zoom(false)
        .allow_drag(false)
        .allow_scroll(false)
        .allow_zoom(false)
        .include_x(*control_x.input.range.start())
        .include_x(*control_x.input.range.end())
        .include_y(*control_y.input.range.start())
        .include_y(*control_y.input.range.end())
        .legend(Legend::default())
        .show_axes([false, false])
        .width(size)
        .height(size)
        .show(ui, |plot_ui| {
            plot_ui.vline(VLine::new(control_x.value).name(control_x_name));
            plot_ui.hline(HLine::new(control_y.value).name(control_y_name));
        });
}

fn tuner(
    ui: &mut egui::Ui,
    width: f32,
    height: f32,
    settings: &Settings,
    plot_name: &str,
    note_raw: f32,
    note_tuned: f32,
) {
    let scale = settings.scale_notes();
    let closest = crate::music_theory::closest_in_scale(note_raw, &scale);
    let closest = scale
        .get(closest)
        .map(|closest| closest.into_byte() as f32)
        .unwrap_or_else(|| note_raw.round());

    // hack: force the include_x/include_y to recenter on root note change
    let plot_id = format!("{plot_name}{closest}");
    egui::plot::Plot::new(plot_id)
        .allow_boxed_zoom(false)
        .allow_drag(false)
        .allow_scroll(false)
        .allow_zoom(false)
        .include_y(closest - 2.0)
        .include_y(closest + 2.0)
        .include_x(-1.0)
        .include_x(1.0)
        .y_grid_spacer(move |input| {
            ((input.bounds.0.floor() as u8)..=(input.bounds.1.ceil() as u8))
                .into_iter()
                .map(|n| {
                    let note = MidiNote::from_byte(n);
                    let step_size = if scale.contains(&note) { 5.0 } else { 1.0 };
                    GridMark {
                        value: n as f64,
                        step_size,
                    }
                })
                .collect()
        })
        .show_axes([false, true])
        .y_axis_formatter(note_formatter)
        .width(width)
        .height(height)
        .show(ui, |plot_ui| {
            plot_ui.hline(HLine::new(note_raw).name("Note"));
            plot_ui.hline(HLine::new(note_tuned).name("Note (Tuned)"));
        });
}
