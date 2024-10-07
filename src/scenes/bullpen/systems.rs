use crate::prelude::*;

pub(crate) fn spawn_camera(mut commands: Commands) {
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
    commands.spawn((
        BlueprintInfo::from_path("levels/TheBullpen.glb"),
        SpawnBlueprint,
        HideUntilReady,
        GameWorldTag,
    ));
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
}

pub(crate) fn launch_ball(
    mut commands: Commands,
    selected_pitch_parameters: Res<SelectedPitchParameters>,
    mut ev_activate_aerodynamics: EventWriter<ActivateAerodynamicsEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let gyro_pole = selected_pitch_parameters.0.gyro_pole;
    let spin_efficiency: f32 = selected_pitch_parameters.0.spin_efficiency;
    let velocity: f32 = 96. * MPH_TO_FTS;
    let spin_rate: f32 = 2400.;
    let seam_z_angle: f32 = std::f32::consts::PI / 2.;
    let tilt = Tilt::from_hour_mintes(12, 0);

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
    info!(
        "{:?}",
        (-Vec3::Y * velocity).from_baseball_coord_to_bevy().length()
    );
    let entity = commands
        .spawn((
            BaseballMarker,
            Name::new("ball"),
            //
            BaseballFlightBundle::default(),
            //
            ExternalForce::default(),
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(
                0.48, 1.82, 16.764,
            ))),
            Velocity {
                linvel: (-Vec3::Y * velocity).from_baseball_coord_to_bevy(),
                angvel: spin.from_baseball_coord_to_bevy(),
            },
            //
            Restitution {
                coefficient: 0.546,
                combine_rule: CoefficientCombineRule::Min,
            },
            //
            InheritedVisibility::VISIBLE,
        ))
        .with_children(|child| {
            child.spawn((PbrBundle {
                mesh: meshes.add(Sphere::new(0.03).mesh().uv(32, 18)),
                material: materials.add(StandardMaterial {
                    base_color: Color::WHITE,
                    perceptual_roughness: 1.0,
                    ..default()
                }),
                ..default()
            },));
        })
        .id();

    ev_activate_aerodynamics.send(ActivateAerodynamicsEvent {
        entity,
        seam_y_angle: 0.,
        seam_z_angle,
    });
}
