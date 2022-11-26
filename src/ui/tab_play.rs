use std::{f64::consts::TAU, ops::RangeInclusive};

use egui::{
    plot::{
        uniform_grid_spacer, Bar, BarChart, GridMark, Legend, Line, MarkerShape, PlotPoint,
        PlotPoints, Points, VLine,
    },
    remap, Color32, TextStyle, Widget,
};
use nalgebra::Vector2;
use staff::midi::MidiNote;

use crate::{
    controls::{self},
    settings::Preset,
    ui::KeyboardEditMode,
};

pub struct TabPlay<'a> {
    controls: &'a controls::Controls,
    preset: &'a mut Preset,
    lead_volume: f32,
    lead_chord_notes: &'a [f32; 4],
    lead_chord_volumes: &'a [f32; 4],
    raw_note: f32,
    filter_cutoff: f32,
    filter_resonance: f32,
    pitch_xy: (f32, f32),
    chords_number: f32,
}

impl<'a> TabPlay<'a> {
    #[allow(clippy::too_many_arguments)] // could review this...
    pub fn new(
        controls: &'a controls::Controls,
        preset: &'a mut Preset,
        lead_volume: f32,
        lead_chord_notes: &'a [f32; 4],
        lead_chord_volumes: &'a [f32; 4],
        raw_note: f32,
        filter_cutoff: f32,
        filter_resonance: f32,
        pitch_xy: (f32, f32),
        chords_number: f32,
    ) -> Self {
        Self {
            controls,
            preset,
            lead_volume,
            lead_chord_notes,
            lead_chord_volumes,
            raw_note,
            filter_cutoff,
            filter_resonance,
            pitch_xy,
            chords_number,
        }
    }
}

impl Widget for TabPlay<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical(|ui| {
            ui.add(crate::ui::Keyboard::new(
                self.lead_chord_notes,
                self.lead_chord_volumes,
                self.preset,
                KeyboardEditMode::Drone,
            ));
            ui.separator();
            ui.horizontal(|ui| {
                let height = 235.0;
                ui.add_space(8.0);
                self.chords_number(ui, 75.0, height, "chords");
                ui.add_space(8.0);
                self.pitch_plot("pitch", ui, height);
                ui.add_space(16.0);
                self.filter_plot(ui, height, "filter");
                ui.add_space(8.0);
                self.volume(ui, 75.0, height, "volume");
                ui.add_space(8.0);
            });
            self.tuner(ui, 670.0, 38.0, "tuner");
        })
        .response
    }
}

impl<'a> TabPlay<'a> {
    fn circle(&self, radius: f32, center: Vector2<f32>) -> Line {
        let n = 512;
        let r = radius as f64;
        let circle_points: PlotPoints = (0..=n)
            .map(|i| {
                let t = remap(i as f64, 0.0..=(n as f64), 0.0..=TAU);
                [r * t.cos() + center.x as f64, r * t.sin() + center.y as f64]
            })
            .collect();
        Line::new(circle_points)
    }

    fn pitch_plot(&self, name: &str, ui: &mut egui::Ui, size: f32) {
        let note_range = self.preset.note_range_f();
        let xy_range = (note_range.end() - note_range.start()) * 0.9;

        let raw_coordinates = Vector2::new(self.pitch_xy.0, self.pitch_xy.1);
        let raw_coordinates_direction = raw_coordinates.normalize();
        let root = self.preset.root_note();
        let scale = self.preset.restricted_scale();
        egui::plot::Plot::new(name)
            .allow_boxed_zoom(false)
            .allow_drag(false)
            .allow_scroll(false)
            .allow_zoom(false)
            .include_x(-1.0)
            .include_x(xy_range)
            .include_y(1.0)
            .include_y(-xy_range)
            .x_grid_spacer(uniform_grid_spacer(|_| [12.0, 1.0, 1.0]))
            .y_grid_spacer(uniform_grid_spacer(|_| [12.0, 1.0, 1.0]))
            .show_axes([false, false]) // would be nice to have, but have to deal with inverted axes first
            .width(size)
            .height(size)
            .legend(Legend::default().text_style(TextStyle::Small))
            .show(ui, |plot_ui| {
                plot_ui.points(
                    Points::new(PlotPoints::Owned(vec![PlotPoint::new(
                        raw_coordinates.x,
                        raw_coordinates.y,
                    )]))
                    .shape(MarkerShape::Plus)
                    .radius(6.0)
                    .name("Note"),
                );

                for (note, volume) in self.lead_chord_notes.iter().zip(self.lead_chord_volumes) {
                    let coordinates = raw_coordinates_direction * (*note_range.end() - note);
                    plot_ui.points(
                        Points::new(PlotPoints::Owned(vec![PlotPoint::new(
                            coordinates.x,
                            coordinates.y,
                        )]))
                        .shape(MarkerShape::Circle)
                        .radius(6.0 * volume),
                    );
                }

                for note in scale {
                    let width = if note.pitch() == root.pitch() {
                        3.0
                    } else {
                        1.0
                    };
                    let circle = self
                        .circle(
                            *note_range.end() - note.into_byte() as f32,
                            Vector2::new(0.0, 0.0),
                        )
                        .color(Color32::from_rgb(100, 200, 100))
                        .width(width);
                    plot_ui.line(circle);
                }
            });
    }

    fn volume(&self, ui: &mut egui::Ui, width: f32, height: f32, plot_name: &str) {
        egui::plot::Plot::new(plot_name)
            .allow_boxed_zoom(false)
            .allow_drag(false)
            .allow_scroll(false)
            .allow_zoom(false)
            .include_y(0.0)
            .include_y(1.0)
            .include_x(-1.0)
            .include_x(1.0)
            .show_axes([false, false])
            .width(width)
            .height(height)
            .legend(Legend::default().text_style(TextStyle::Small))
            .show(ui, |plot_ui| {
                plot_ui.bar_chart(
                    BarChart::new(vec![Bar::new(0.0, self.lead_volume.into())])
                        .width(2.0)
                        .name("Vol."),
                );
            });
    }

    fn chords_number(&self, ui: &mut egui::Ui, width: f32, height: f32, plot_name: &str) {
        egui::plot::Plot::new(plot_name)
            .allow_boxed_zoom(false)
            .allow_drag(false)
            .allow_scroll(false)
            .allow_zoom(false)
            .include_y(1.0)
            .include_y(5.0)
            .include_x(-1.0)
            .include_x(1.0)
            .show_axes([false, true])
            .width(width)
            .height(height)
            .legend(Legend::default().text_style(TextStyle::Small))
            .show(ui, |plot_ui| {
                plot_ui.bar_chart(
                    BarChart::new(vec![Bar::new(0.0, (self.chords_number + 1.0).into())])
                        .width(2.0)
                        .name("Chord"),
                );
            });
    }

    fn tuner(&self, ui: &mut egui::Ui, width: f32, height: f32, plot_name: &str) {
        let scale = self.preset.restricted_scale();
        let scale_window = self.preset.restricted_scale_floating_window();
        let note_raw = self.raw_note;
        let note_tuned = self.lead_chord_notes[0];
        let closest = scale_window.closest_in_scale(note_raw);

        // hack: force the include_x/include_y to recenter on root note change
        let plot_id = format!("{plot_name}{closest}");
        egui::plot::Plot::new(plot_id)
            .allow_boxed_zoom(false)
            .allow_drag(false)
            .allow_scroll(false)
            .allow_zoom(false)
            .include_x(closest - 2.0)
            .include_x(closest + 2.0)
            .include_y(-1.0)
            .include_y(1.0)
            .y_grid_spacer(move |input| {
                ((input.bounds.0.floor() as u8)..=(input.bounds.1.ceil() as u8))
                    .into_iter()
                    .map(|n| {
                        let note = MidiNote::from_byte(n);
                        let step_size = if scale.contains(&note) { 5.0 } else { 1.0 };
                        GridMark {
                            step_size,
                            value: n as f64,
                        }
                    })
                    .collect()
            })
            .show_axes([true, false])
            .x_axis_formatter(note_formatter)
            .width(width)
            .height(height)
            .show(ui, |plot_ui| {
                plot_ui.vline(VLine::new(note_raw).name("Note"));
                plot_ui.vline(VLine::new(note_tuned).name("Note (Tuned)"));
            });
    }

    fn filter_plot(&self, ui: &mut egui::Ui, size: f32, plot_name: &str) {
        let cutoff = &self.controls.cutoff_note;
        let resonance = &self.controls.resonance;
        egui::plot::Plot::new(plot_name)
            .allow_boxed_zoom(false)
            .allow_drag(false)
            .allow_scroll(false)
            .allow_zoom(false)
            .include_x(*cutoff.input.range.start())
            .include_x(*cutoff.input.range.end())
            .include_y(*resonance.input.range.start())
            .include_y(*resonance.input.range.end())
            .legend(Legend::default().text_style(TextStyle::Small))
            .show_axes([false, false])
            .width(size)
            .height(size)
            .show(ui, |plot_ui| {
                plot_ui.points(
                    Points::new(vec![[
                        self.filter_cutoff as f64,
                        self.filter_resonance as f64,
                    ]])
                    .radius(10.0)
                    .name("Filter"),
                );
            });
    }
}

fn note_formatter(note: f64, _range: &RangeInclusive<f64>) -> String {
    MidiNote::from_byte(note as u8).to_string()
}
