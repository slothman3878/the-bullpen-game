use crate::prelude::*;

pub(crate) fn spawn_baseball(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        BlueprintInfo::from_path("blueprints/Baseball.glb"),
        SpawnBlueprint,
        HideUntilReady,
        GameWorldTag,
    ));
}

// render layer 0 has the scene
// render layer 1 has the baseball preview

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
    egui::Window::new("menu").show(ctx, |ui| {
        egui::Grid::new("parameters").show(ui, |ui| {
            ui.label("speed (mph)");
            egui::Slider::new(&mut selected_pitch_parameters.0.speed, 30.0_f32..=110.0_f32).ui(ui);
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

            let (hr, _) = selected_pitch_parameters.0.tilt.to_hour_minutes();
            let mut selected_hr = hr;
            ui.label("tilt");
            egui::Slider::new(&mut selected_hr, 0..=12).ui(ui);
            ui.end_row();
            selected_pitch_parameters.0.tilt = Tilt::from_hour_mintes(selected_hr, 0);
        });
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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
                child.spawn((
                    PbrBundle {
                        mesh: meshes.add(Sphere::new(0.03).mesh().uv(32, 18)),
                        material: materials.add(StandardMaterial {
                            base_color: Color::WHITE,
                            perceptual_roughness: 1.0,
                            ..default()
                        }),
                        ..default()
                    },
                    RenderLayers::from_layers(&[0]),
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
