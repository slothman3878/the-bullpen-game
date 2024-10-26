use crate::prelude::*;

pub(crate) fn params_menu(
    mut selected_pitch_parameters: ResMut<SelectedPitchParameters>,
    mut contexts: EguiContexts,
) {
    let ctx = contexts.ctx_mut();
    egui::Window::new("menu").min_width(300.0).show(ctx, |ui| {
        ui.add_space(10.0); // Add some space at the top
        ui.with_layout(
            egui::Layout::top_down_justified(egui::Align::Center),
            |ui| {
                egui::Grid::new("parameters")
                    .spacing([50.0, 50.0])
                    .show(ui, |ui| {
                        ui.label("speed (mph)");
                        egui::Slider::new(
                            &mut selected_pitch_parameters.0.speed,
                            30.0_f32..=110.0_f32,
                        )
                        .ui(ui);
                        ui.end_row();

                        ui.label("spin (rpm)");
                        egui::Slider::new(
                            &mut selected_pitch_parameters.0.spin_rate,
                            500.0_f32..=3000.0_f32,
                        )
                        .ui(ui);
                        ui.end_row();

                        ui.label("spin efficiency (%)");
                        egui::Slider::new(
                            &mut selected_pitch_parameters.0.spin_efficiency,
                            0.0_f32..=1.0_f32,
                        )
                        .ui(ui);
                        ui.end_row();

                        ui.label("gyro pole");
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                            ui.selectable_value(
                                &mut selected_pitch_parameters.0.gyro_pole,
                                GyroPole::Left,
                                "Left",
                            );
                            ui.selectable_value(
                                &mut selected_pitch_parameters.0.gyro_pole,
                                GyroPole::Right,
                                "Right",
                            );
                        });
                        ui.end_row();

                        ui.label("tilt");
                        ui.vertical(|ui| {
                            let (hr, _) = selected_pitch_parameters.0.tilt.to_hour_minutes();
                            let mut selected_hr = hr;

                            // Clock visualization
                            let (rect, _) = ui.allocate_exact_size(
                                egui::vec2(100.0, 100.0),
                                egui::Sense::hover(),
                            );
                            let painter = ui.painter();

                            // Draw clock face
                            painter.circle_stroke(
                                rect.center(),
                                45.0,
                                egui::Stroke::new(2.0, egui::Color32::WHITE),
                            );

                            // Draw clock hand
                            let angle = -2.0 * std::f32::consts::PI * (selected_hr as f32 / 12.0)
                                + std::f32::consts::PI / 2.0;
                            let hand_end =
                                rect.center() + egui::vec2(angle.cos() * 40.0, -angle.sin() * 40.0);
                            painter.line_segment(
                                [rect.center(), hand_end],
                                egui::Stroke::new(2.0, egui::Color32::RED),
                            );

                            // Draw 12 o'clock marker
                            let marker_12 = rect.center() + egui::vec2(0.0, -45.0);
                            painter.circle_filled(marker_12, 3.0, egui::Color32::WHITE);

                            // Add some space between the clock and the slider
                            ui.add_space(10.0);

                            // Add the slider beneath the clock
                            egui::Slider::new(&mut selected_hr, 1..=12).ui(ui);
                            selected_pitch_parameters.0.tilt =
                                Tilt::from_hour_mintes(selected_hr, 0);
                        });
                        ui.end_row();

                        ui.label("seam orientation");
                        egui::Grid::new("seam orientation").show(ui, |ui| {
                            ui.label("y angle (°)");
                            let mut seam_y_angle_deg =
                                selected_pitch_parameters.0.seam_y_angle.to_degrees();
                            egui::Slider::new(&mut seam_y_angle_deg, 0.0_f32..=180.).ui(ui);
                            ui.end_row();
                            selected_pitch_parameters.0.seam_y_angle =
                                seam_y_angle_deg.to_radians();

                            ui.label("z angle (°)");
                            let mut seam_z_angle_deg =
                                selected_pitch_parameters.0.seam_z_angle.to_degrees();
                            egui::Slider::new(&mut seam_z_angle_deg, 0.0_f32..=180.).ui(ui);
                            ui.end_row();
                            selected_pitch_parameters.0.seam_z_angle =
                                seam_z_angle_deg.to_radians();
                        });
                    });
            },
        );
        ui.add_space(10.0); // Add some space at the bottom
    });
}
