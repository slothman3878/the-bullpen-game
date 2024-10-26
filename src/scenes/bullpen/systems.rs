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

// pub(crate) fn baseball_preview_3d(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     mut contexts: EguiContexts,
//     mut images: ResMut<Assets<Image>>,
//     selected_pitch_parameters: Res<SelectedPitchParameters>,
//     query: Query<(Entity, &Handle<Image>), With<BaseballPreviewMarker>>,
// ) {
//     let ctx = contexts.ctx_mut();

//     egui::Window::new("Baseball 3D Preview")
//         .default_size([300.0, 300.0])
//         .show(ctx, |ui| {
//             let (rect, response) =
//                 ui.allocate_exact_size(egui::vec2(280.0, 280.0), egui::Sense::drag());

//             // Create or update the 3D scene
//             if query.is_empty() {
//                 // Create a new 3D scene
//                 let size = rect.size();
//                 let image_handle = render_baseball_3d(
//                     &mut commands,
//                     &mut meshes,
//                     &mut materials,
//                     &mut images,
//                     size.x as u32,
//                     size.y as u32,
//                     &selected_pitch_parameters,
//                 );
//                 let texture_id = contexts.add_image(image_handle.clone());
//                 ui.image(texture_id, size);
//             } else {
//                 // Update existing 3D scene
//                 for (entity, image_handle) in query.iter() {
//                     update_baseball_3d(&mut commands, entity, &selected_pitch_parameters);
//                     let texture_id = contexts.add_image(image_handle.clone());
//                     ui.image(texture_id, rect.size());
//                 }
//             }

//             // Handle user interaction (rotation)
//             if response.dragged() {
//                 // Implement rotation logic here
//             }
//         });
// }

// fn render_baseball_3d(
//     commands: &mut Commands,
//     meshes: &mut Assets<Mesh>,
//     materials: &mut Assets<StandardMaterial>,
//     images: &mut Assets<Image>,
//     width: u32,
//     height: u32,
//     params: &SelectedPitchParameters,
// ) -> Handle<Image> {
//     // Create a new render target
//     let size = Extent3d {
//         width,
//         height,
//         ..default()
//     };
//     let mut image = Image {
//         texture_descriptor: TextureDescriptor {
//             size,
//             dimension: TextureDimension::D2,
//             format: TextureFormat::Rgba8UnormSrgb,
//             usage: TextureUsages::TEXTURE_BINDING
//                 | TextureUsages::COPY_DST
//                 | TextureUsages::RENDER_ATTACHMENT,
//             ..default()
//         },
//         ..default()
//     };
//     image.resize(size);

//     let image_handle = images.add(image);

//     // Create a 3D scene
//     let camera = commands
//         .spawn((
//             Camera3dBundle {
//                 transform: Transform::from_translation(Vec3::new(0.0, 0.0, 5.0))
//                     .looking_at(Vec3::ZERO, Vec3::Y),
//                 ..default()
//             },
//             RenderLayers::from_layers(&[2]),
//         ))
//         .id();

//     let baseball = commands
//         .spawn((
//             PbrBundle {
//                 mesh: meshes.add(Mesh::from(shape::UVSphere {
//                     radius: 1.0,
//                     sectors: 32,
//                     stacks: 16,
//                 })),
//                 material: materials.add(StandardMaterial {
//                     base_color: Color::WHITE,
//                     ..default()
//                 }),
//                 transform: Transform::from_rotation(Quat::from_euler(
//                     EulerRot::YXZ,
//                     params.0.seam_y_angle,
//                     0.0,
//                     params.0.seam_z_angle,
//                 )),
//                 ..default()
//             },
//             BaseballPreviewMarker,
//         ))
//         .id();

//     // Set up the render pipeline
//     commands.spawn(Camera2dBundle {
//         camera: Camera {
//             target: RenderTarget::Image(image_handle.clone()),
//             ..default()
//         },
//         ..default()
//     });

//     image_handle
// }

// fn update_baseball_3d(commands: &mut Commands, entity: Entity, params: &SelectedPitchParameters) {
//     commands
//         .entity(entity)
//         .insert(Transform::from_rotation(Quat::from_euler(
//             EulerRot::YXZ,
//             params.0.seam_y_angle,
//             0.0,
//             params.0.seam_z_angle,
//         )));
// }

// #[derive(Component)]
// struct BaseballPreviewMarker;
