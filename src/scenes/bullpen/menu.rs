use crate::prelude::*;
use std::f32::consts::PI;

#[derive(PartialEq, Debug, Default)]
pub(crate) enum MenuTab {
    #[default]
    Parameters,
    Controls,
    Settings,
}

#[derive(Debug, Resource, Default)]
pub(crate) struct MenuState {
    pub visibility: bool,
    pub selected_tab: MenuTab,
}

pub(crate) fn menu_visibility_is(visibility: bool) -> impl FnMut(Res<MenuState>) -> bool + Clone {
    move |menu_visibility| menu_visibility.visibility == visibility
}

pub(crate) fn toggle_menu_visibility(mut menu_visibility: ResMut<MenuState>) {
    menu_visibility.visibility = !menu_visibility.visibility;
}

pub(crate) fn params_menu(
    mut contexts: EguiContexts,
    mut selected_pitch_parameters: ResMut<SelectedPitchParameters>,
    mut active_batter_tracker: ResMut<ActiveBatterTracker>,
    baseball_preview_image: Res<BaseballPreviewImage>,
    mut menu_state: ResMut<MenuState>,
    mut exit: EventWriter<AppExit>,
) {
    let opt_cube_preview_texture_id = contexts.image_id(&baseball_preview_image);

    let ctx = contexts.ctx_mut();

    egui::Window::new("menu").min_width(600.0).show(ctx, |ui| {
        ui.set_min_width(600.0);

        // Add tabs at the top of the window
        egui::TopBottomPanel::top("tabs").show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut menu_state.selected_tab,
                    MenuTab::Parameters,
                    "Parameters",
                );
                ui.selectable_value(&mut menu_state.selected_tab, MenuTab::Controls, "Controls");
                ui.selectable_value(&mut menu_state.selected_tab, MenuTab::Settings, "Settings");
            });
        });

        egui::Frame::none()
            .inner_margin(egui::Margin::symmetric(20.0, 10.0)) // Add padding to all sides
            .show(ui, |ui| {
                match menu_state.selected_tab {
                    MenuTab::Parameters => {
                        ui.horizontal(|ui| {
                            // Parameters section
                            ui.vertical(|ui| {
                                ui.add_space(10.0);

                                egui::Grid::new("parameters").spacing([50.0, 50.0]).show(
                                    ui,
                                    |ui| {
                                        // in cm
                                        let mut batter_height = active_batter_tracker.height * 100.;
                                        ui.label("batter height (cm)");
                                        egui::Slider::new(
                                            &mut batter_height,
                                            150.0_f32..=240.0_f32,
                                        )
                                        .ui(ui);
                                        ui.end_row();
                                        if (batter_height / 100. - active_batter_tracker.height)
                                            .abs()
                                            >= 0.01
                                        {
                                            active_batter_tracker.height = batter_height / 100.;
                                        }

                                        ui.label("Pitching Arm");
                                        ui.with_layout(
                                            egui::Layout::left_to_right(egui::Align::TOP),
                                            |ui| {
                                                ui.selectable_value(
                                                    &mut selected_pitch_parameters.0.pitching_arm,
                                                    PitchingArm::Lefty,
                                                    "Lefty",
                                                );
                                                ui.selectable_value(
                                                    &mut selected_pitch_parameters.0.pitching_arm,
                                                    PitchingArm::Righty,
                                                    "Righty",
                                                );
                                            },
                                        );
                                        ui.end_row();

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
                                        ui.with_layout(
                                            egui::Layout::left_to_right(egui::Align::TOP),
                                            |ui| {
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
                                            },
                                        );
                                        ui.end_row();

                                        ui.label("tilt");
                                        ui.vertical(|ui| {
                                            let (hr, min) =
                                                selected_pitch_parameters.0.tilt.to_hour_minutes();
                                            let mut selected_time = hr as f32 + min as f32 / 60.0;

                                            // Clock visualization
                                            let (rect, response) = ui.allocate_exact_size(
                                                egui::vec2(100.0, 100.0),
                                                egui::Sense::click_and_drag(),
                                            );
                                            let painter = ui.painter();

                                            // Draw clock face
                                            painter.circle_stroke(
                                                rect.center(),
                                                45.0,
                                                egui::Stroke::new(2.0, egui::Color32::WHITE),
                                            );

                                            // Handle user interaction
                                            if response.dragged() || response.clicked() {
                                                let mouse_pos = response
                                                    .interact_pointer_pos()
                                                    .unwrap_or(rect.center());
                                                let vector = mouse_pos - rect.center();
                                                let angle = (vector.y.atan2(vector.x) + PI * 2.)
                                                    % (PI * 2.);
                                                selected_time =
                                                    (angle / (2.0 * PI) * 12.0 + 3.0) % 12.0;
                                                if selected_time.floor() == 0. {
                                                    selected_time += 12.0;
                                                }
                                            }

                                            // Draw clock hand
                                            let angle =
                                                -2.0 * PI * (selected_time / 12.0) + PI / 2.0;
                                            let hand_end = rect.center()
                                                + egui::vec2(
                                                    angle.cos() * 40.0,
                                                    -angle.sin() * 40.0,
                                                );
                                            painter.line_segment(
                                                [rect.center(), hand_end],
                                                egui::Stroke::new(2.0, egui::Color32::RED),
                                            );

                                            // get hr and min
                                            let selected_hr = selected_time.floor() as i8;
                                            let selected_min =
                                                ((selected_time - selected_hr as f32) * 60.0)
                                                    .floor()
                                                    as i8;

                                            // Draw 12 o'clock marker
                                            let marker_12 = rect.center() + egui::vec2(0.0, -45.0);
                                            painter.circle_filled(
                                                marker_12,
                                                3.0,
                                                egui::Color32::WHITE,
                                            );

                                            // Add some space between the clock and the text
                                            ui.add_space(10.0);

                                            // Display the selected hour as text
                                            ui.label(format!(
                                                "{}:{:02}",
                                                selected_hr, selected_min
                                            ));

                                            // Update the tilt in the SelectedPitchParameters
                                            selected_pitch_parameters.0.tilt =
                                                Tilt::from_hour_mintes(selected_hr, selected_min)
                                                    .expect("invalid tilt params".into());
                                        });
                                        ui.end_row();
                                    },
                                );
                            });

                            ui.add_space(20.0); // Add some space between sections

                            // Preview section and seam orientation
                            ui.vertical(|ui| {
                                if let Some(cube_preview_texture_id) = opt_cube_preview_texture_id {
                                    ui.image(egui::load::SizedTexture::new(
                                        cube_preview_texture_id,
                                        egui::vec2(300., 300.),
                                    ));
                                }

                                ui.add_space(10.0); // Add some space between preview and sliders

                                ui.label("Seam Orientation");
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
                        });
                    }
                    MenuTab::Controls => {
                        ui.add_space(20.0);
                        ui.vertical(|ui| {
                            ui.heading("Controls");
                            ui.add_space(10.0);

                            ui.label("• Aim with mouse");
                            ui.label("• Right Mouse Button to aim, then release to launch");
                            ui.label("• Press R to reset ball");
                        });
                    }
                    MenuTab::Settings => {
                        ui.add_space(20.0);
                        if ui.button("Exit Game").clicked() {
                            exit.send(AppExit::Success);
                        }
                    }
                }

                ui.add_space(10.0); // Add some space at the bottom
            });
    });
}

pub(crate) fn update_baseball_preview_3d(
    selected_pitch_parameters: Res<SelectedPitchParameters>,
    mut query_baseball_preview: Query<&mut Transform, With<PreviewPassBaseballMarker>>,
) {
    if let Ok(mut transform) = query_baseball_preview.get_single_mut() {
        let seam_y_angle = selected_pitch_parameters.0.seam_y_angle;
        let seam_z_angle = selected_pitch_parameters.0.seam_z_angle;
        let rot =
            Quat::from_rotation_y(-seam_y_angle).mul_quat(Quat::from_rotation_z(seam_z_angle));
        *transform = transform.with_rotation(rot);
    }
}
