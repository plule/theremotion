use std::{f64::consts::TAU, ops::RangeInclusive};

use egui::{ecolor::Hsva, remap, Color32, TextStyle, Widget};
use egui_plot::{
    uniform_grid_spacer, Bar, BarChart, GridMark, Legend, Line, MarkerShape, PlotPoint, PlotPoints,
    Points, VLine,
};
use nalgebra::Vector2;
use staff::midi::MidiNote;

use crate::{
    controls::{self},
    settings::{Handedness, Settings},
    solfege::MidiNoteF,
    ui::KeyboardEditMode,
};

/// Content of the play tab
pub struct TabPlay<'a> {
    pub controls: &'a controls::Controls,
    pub settings: &'a mut Settings,
    pub lead_volume: f32,
    pub lead_chord_notes: &'a [MidiNoteF; 4],
    pub lead_chord_volumes: &'a [f32; 4],
    pub raw_note: MidiNoteF,
    pub filter_cutoff: f32,
    pub filter_resonance: f32,
    pub pitch_xy: (f32, f32),
    pub chords_number: f32,
    pub autotune_amount: f32,
    pub strum_ready: bool,
    pub trumpet_strength: f32,
}

impl Widget for TabPlay<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical(|ui| {
            ui.add(crate::ui::Keyboard::new(
                self.lead_chord_notes,
                self.lead_chord_volumes,
                &mut self.settings.current_preset,
                KeyboardEditMode::Drone,
            ));
            ui.separator();
            let available_size = ui.available_size();
            let tuner_height = 38.0;
            ui.horizontal(|ui| {
                let shortspace = 8.0;
                let bigspace = 16.0;
                let slider_width = 75.0;

                let xy_size = f32::min(
                    // Remove the tuner from the available height
                    available_size.y - tuner_height,
                    // Remove the sliders and spaces from the available width. Divide by two for the two xy plots
                    (available_size.x - slider_width * 2.0 - shortspace * 4.0 - bigspace) / 2.0,
                ) - 3.0; // Keep some space for the separators

                let chord_number = self.chords_number("chords", slider_width, xy_size);
                let pitch_plot = self.pitch_plot("pitch", xy_size);
                let filter_plot = self.filter_plot("filter", xy_size);
                let volume_plot = self.volume("volume", slider_width, xy_size);
                match self.settings.system.handedness {
                    Handedness::RightHanded => {
                        ui.add_space(shortspace);
                        ui.add(volume_plot);
                        ui.add_space(shortspace);
                        ui.add(filter_plot);
                        ui.add_space(bigspace);
                        ui.add(pitch_plot);
                        ui.add_space(shortspace);
                        ui.add(chord_number);
                        ui.add_space(shortspace);
                    }
                    Handedness::LeftHanded => {
                        ui.add_space(shortspace);
                        ui.add(chord_number);
                        ui.add_space(shortspace);
                        ui.add(pitch_plot);
                        ui.add_space(bigspace);
                        ui.add(filter_plot);
                        ui.add_space(shortspace);
                        ui.add(volume_plot);
                        ui.add_space(shortspace);
                    }
                };
            });
            ui.add(self.tuner("tuner", ui.available_width(), tuner_height));
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

    fn pitch_plot(&self, name: &'a str, size: f32) -> impl egui::Widget + '_ {
        move |ui: &mut egui::Ui| {
            let preset = &self.settings.current_preset;
            let note_range = preset.note_range_f();
            let xy_range = (*note_range.end() - *note_range.start()) * 0.9;

            let raw_coordinates = Vector2::new(self.pitch_xy.0, self.pitch_xy.1);
            let raw_coordinates_direction = raw_coordinates.normalize();
            let root = preset.root_note();
            let scale = preset.restricted_scale();
            // Even though only the distance is taken in account,
            // in right handed mode, the hand is generally on the left of the antenna
            // and in left handed mode, the hand is on the right
            let x_range = match self.settings.system.handedness {
                Handedness::RightHanded => -xy_range,
                Handedness::LeftHanded => xy_range,
            };
            // In any case, the hand is always on the negative y
            let y_range = -xy_range;
            egui_plot::Plot::new(format!("{name}{x_range}"))
                .allow_boxed_zoom(false)
                .allow_drag(false)
                .allow_scroll(false)
                .allow_zoom(false)
                .include_x(-1.0)
                .include_x(x_range.semitones())
                .include_y(1.0)
                .include_y(y_range.semitones())
                .x_grid_spacer(uniform_grid_spacer(|_| [12.0, 1.0, 1.0]))
                .y_grid_spacer(uniform_grid_spacer(|_| [12.0, 1.0, 1.0]))
                .show_axes([false, false]) // would be nice to have, but have to deal with inverted axes first
                .width(size)
                .height(size)
                .legend(Legend::default().text_style(TextStyle::Small))
                .show(ui, |plot_ui| {
                    for note in scale {
                        let width = if note.pitch() == root.pitch() {
                            3.0
                        } else {
                            1.0
                        };
                        let circle = self
                            .circle(
                                note_range.end().note() - note.into_byte() as f32,
                                Vector2::new(0.0, 0.0),
                            )
                            .color(Color32::from_rgb(100, 200, 100))
                            .width(width);
                        plot_ui.line(circle);
                    }

                    plot_ui.points(
                        Points::new(PlotPoints::Owned(vec![PlotPoint::new(
                            raw_coordinates.x,
                            raw_coordinates.y,
                        )]))
                        .shape(MarkerShape::Plus)
                        .radius(6.0)
                        .name("Note"),
                    );

                    let color = intensity_color(self.autotune_amount);
                    for (note, volume) in self.lead_chord_notes.iter().zip(self.lead_chord_volumes)
                    {
                        let coordinates =
                            raw_coordinates_direction * (*note_range.end() - *note).semitones();
                        plot_ui.points(
                            Points::new(PlotPoints::Owned(vec![PlotPoint::new(
                                coordinates.x,
                                coordinates.y,
                            )]))
                            .shape(MarkerShape::Circle)
                            .color(color)
                            .radius(6.0 * volume),
                        );
                    }
                })
                .response
        }
    }

    fn volume(&self, plot_name: &'a str, width: f32, height: f32) -> impl egui::Widget + '_ {
        move |ui: &mut egui::Ui| {
            egui_plot::Plot::new(plot_name)
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
                })
                .response
        }
    }

    fn chords_number(&self, plot_name: &'a str, width: f32, height: f32) -> impl egui::Widget + '_ {
        move |ui: &mut egui::Ui| {
            egui_plot::Plot::new(plot_name)
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
                })
                .response
        }
    }

    fn tuner(&self, plot_name: &'a str, width: f32, height: f32) -> impl egui::Widget + '_ {
        move |ui: &mut egui::Ui| {
            let preset = &self.settings.current_preset;
            let scale = preset.restricted_scale();
            let scale_window = preset.restricted_scale_floating_window();
            let note_raw = self.raw_note;
            let note_tuned = self.lead_chord_notes[0];
            let closest = scale_window.closest_in_scale(note_raw);

            // hack: force the include_x/include_y to recenter on root note change
            let plot_id = format!("{plot_name}{}", closest.note());
            egui_plot::Plot::new(plot_id)
                .allow_boxed_zoom(false)
                .allow_drag(false)
                .allow_scroll(false)
                .allow_zoom(false)
                .include_x(closest.note() - 2.0)
                .include_x(closest.note() + 2.0)
                .include_y(-1.0)
                .include_y(1.0)
                .x_grid_spacer(move |input| {
                    ((input.bounds.0.floor() as u8)..=(input.bounds.1.ceil() as u8))
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
                })
                .response
        }
    }

    fn filter_plot(&self, plot_name: &'a str, size: f32) -> impl egui::Widget + '_ {
        let shape = match self.strum_ready {
            true => MarkerShape::Diamond,
            false => MarkerShape::Circle,
        };
        let color = intensity_color(self.trumpet_strength);
        move |ui: &mut egui::Ui| {
            egui_plot::Plot::new(plot_name)
                .allow_boxed_zoom(false)
                .allow_drag(false)
                .allow_scroll(false)
                .allow_zoom(false)
                .include_x(-1.0)
                .include_x(1.0)
                .include_y(0.0)
                .include_y(1.0)
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
                        .shape(shape)
                        .color(color)
                        .name("Filter"),
                    );
                })
                .response
        }
    }
}

/// Get a color representing an intensity from an input in 0-1
fn intensity_color(value: f32) -> Hsva {
    let saturation = crate::controls::convert_range(value, &(0.0..=1.0), &(0.2..=1.0));
    Hsva::new(18.0 / 360.0, saturation, 0.5, 1.0)
}

fn note_formatter(note: f64, _nchar: usize, _range: &RangeInclusive<f64>) -> String {
    MidiNote::from_byte(note as u8).to_string()
}
