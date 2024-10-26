use crate::prelude::*;

// render layer 0 has the scene
// render layer 1 has the baseball preview

const PI: f32 = std::f32::consts::PI;

pub(crate) fn _spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("fly cam"),
        FlyCam,
        Camera3dBundle {
            camera: Camera {
                is_active: true,
                order: 0,
                ..default()
            },
            transform: Transform::from_xyz(1.316, 2., 23.142),
            ..default()
        },
    ));
}

pub(crate) fn setup_scene(mut commands: Commands) {
    // TODO: need to add render layers to blenvy
    commands.spawn((
        BlueprintInfo::from_path("levels/TheBullpen.glb"),
        SpawnBlueprint,
        HideUntilReady,
        GameWorldTag,
    ));
}

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

#[derive(Debug, Component)]
pub(crate) struct BaseballMarker;

pub(crate) fn despawn_ball(
    mut commands: Commands,
    query_baseball: Query<Entity, With<BaseballMarker>>,
) {
    for baseball in query_baseball.iter() {
        commands.entity(baseball).despawn_recursive();
    }
}

pub(crate) fn spawn_ball(
    mut commands: Commands,
    selected_pitch_parameters: Res<SelectedPitchParameters>,
    query_baseball: Query<Entity, With<BaseballMarker>>,
) {
    if let Ok(_) = query_baseball.get_single() {
        info!("ball already exists");
    } else {
        commands
            .spawn((
                BaseballMarker,
                Name::new("ball"),
                //
                BaseballFlightBundle::default(),
                //
                ExternalForce::default(),
                TransformBundle::from_transform(Transform::from_translation(
                    selected_pitch_parameters.0.starting_point,
                )),
                Velocity::default(),
                //
                Restitution {
                    coefficient: 0.546,
                    combine_rule: CoefficientCombineRule::Min,
                },
                //
                InheritedVisibility::VISIBLE,
                RenderLayers::from_layers(&[0]),
            ))
            .with_children(|child| {
                let seam_y_angle = selected_pitch_parameters.0.seam_y_angle;
                let seam_z_angle = selected_pitch_parameters.0.seam_z_angle;

                let rot = Quat::from_rotation_y(-seam_y_angle)
                    .mul_quat(Quat::from_rotation_z(seam_z_angle));

                child.spawn((
                    BlueprintInfo::from_path("blueprints/Baseball.glb"),
                    SpawnBlueprint,
                    HideUntilReady,
                    RenderLayers::from_layers(&[0]),
                    TransformBundle::from_transform(
                        Transform::from_scale(0.037 * Vec3::new(1., 1.0, 1.0)).with_rotation(rot),
                    ),
                ));
            });
    }
}

pub(crate) fn launch_ball(
    mut selected_pitch_parameters: ResMut<SelectedPitchParameters>,
    rapier_context: Res<RapierContext>,
    camera_query: Query<&GlobalTransform, With<PitcherCameraMarker>>,
    mut query_baseball: Query<(Entity, &mut Velocity), With<BaseballMarker>>,
    mut ev_activate_aerodynamics: EventWriter<ActivateAerodynamicsEvent>,
) {
    if let Ok((entity, mut velocity)) = query_baseball.get_single_mut() {
        if let Ok(camera_global_transform) = camera_query.get_single() {
            let camera_transform = camera_global_transform.compute_transform();
            let ray_origin = camera_transform.translation;
            let ray_dir = camera_transform.rotation.mul_vec3(-Vec3::Z).normalize();
            let max_toi = f32::INFINITY;
            let query = QueryFilter::new();

            let direction = match rapier_context.cast_ray(ray_origin, ray_dir, max_toi, true, query)
            {
                Some((_entity, toi)) => {
                    let aim_point = ray_origin + ray_dir * toi;
                    (aim_point - ray_origin).normalize()
                }
                None => ray_dir,
            };

            selected_pitch_parameters.0.direction = direction;
        }

        let PitchParams {
            gyro_pole,
            spin_efficiency,
            speed,
            spin_rate,
            seam_z_angle,
            tilt,
            starting_point: _,
            direction,
            seam_y_angle,
        } = selected_pitch_parameters.0;

        let fixed_spin_rate = if spin_rate == 0. { 1. } else { spin_rate };

        let gyro = match gyro_pole {
            GyroPole::Left => spin_efficiency.asin(),
            GyroPole::Right => std::f32::consts::PI - spin_efficiency.asin(),
        };

        let spin_x_0 = fixed_spin_rate * (spin_efficiency * tilt.get().sin());
        let spin_y_0 = fixed_spin_rate * gyro.cos(); // ((1. - spin_efficiency.powi(2)).sqrt());
        let spin_z_0 = -fixed_spin_rate * (spin_efficiency * tilt.get().cos());
        let spin = Vec3::new(
            spin_x_0 * RPM_TO_RADS,
            spin_y_0 * RPM_TO_RADS, // - RPM_TO_RAD ???
            spin_z_0 * RPM_TO_RADS,
        );

        info!("speed: {:?}", speed);
        velocity.linvel = direction * speed * 0.44704; // 0.3048;
        velocity.angvel = spin.from_baseball_coord_to_bevy();

        ev_activate_aerodynamics.send(ActivateAerodynamicsEvent {
            entity,
            seam_y_angle,
            seam_z_angle,
        });
    }
}
