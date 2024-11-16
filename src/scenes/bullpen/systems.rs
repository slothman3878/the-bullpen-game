use bevy_rapier3d::rapier::prelude::CollisionEventFlags;

use super::resources::BaseballPreviewImage;
use crate::prelude::*;

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

pub(crate) fn swap_camera(
    game_mode: Res<State<BullpenSceneGameMode>>,
    mut next_game_mode: ResMut<NextState<BullpenSceneGameMode>>,
    mut pitcher_camera_camera: Query<(Entity, &mut ThirdPersonCamera), With<PitcherCameraMarker>>,
    batter_camera_camera: Query<Entity, With<BatterCameraMarker>>,
    mut camera_query: Query<
        &mut Camera,
        Or<(
            // With<FlyCamMarker>,
            With<PitcherCameraMarker>,
            With<BatterCameraMarker>,
        )>,
    >,
) {
    info!("swap camera {:?}", game_mode.get());
    let (pitcher_camera_entity, mut pitcher_aim) = pitcher_camera_camera.single_mut();
    let batter_camera_entity = batter_camera_camera.single();
    let [mut pitcher_camera, mut batter_camera] = camera_query
        .get_many_mut([pitcher_camera_entity, batter_camera_entity])
        .unwrap();
    info!("pitcher camera {:?}", pitcher_camera.is_active);
    info!("batter camera {:?}", batter_camera.is_active);
    info!(
        "pitcher aim {:?} {:?}",
        pitcher_aim.cursor_lock_active, pitcher_aim.cursor_lock_toggle_enabled
    );
    match game_mode.get() {
        BullpenSceneGameMode::Batter => {
            pitcher_aim.cursor_lock_active = true;
            // pitcher_aim.cursor_lock_toggle_enabled = false;
            pitcher_camera.is_active = true;
            batter_camera.is_active = false;
            next_game_mode.set(BullpenSceneGameMode::Pitcher);
        }
        BullpenSceneGameMode::Pitcher => {
            pitcher_aim.cursor_lock_active = false;
            // pitcher_aim.cursor_lock_toggle_enabled = true;
            pitcher_camera.is_active = false;
            batter_camera.is_active = true;
            next_game_mode.set(BullpenSceneGameMode::Batter);
        }
    }
    info!("camera swapped {:?}", game_mode.get());
}

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub(crate) struct StrikezoneSpawnRequestMarker;

pub(crate) fn spawn_strikezone(
    mut commands: Commands,
    active_batter_tracker: Res<ActiveBatterTracker>,
    query_strikezone_spawn_request_marker: Query<Entity, With<StrikezoneSpawnRequestMarker>>,
    mut ev_spawn: EventWriter<SpawnStrikezone>,
) {
    for entity in query_strikezone_spawn_request_marker.iter() {
        info!("strikezone spawn request marker found");
        commands
            .entity(entity)
            .remove::<StrikezoneSpawnRequestMarker>();
        ev_spawn.send(SpawnStrikezone {
            batter_height: active_batter_tracker.height,
        });
    }
}

pub(crate) fn third_person_camera_lock_status(
    query_third_person_camera: Query<&ThirdPersonCamera, With<PitcherCameraMarker>>,
) {
    if let Ok(third_person_camera) = query_third_person_camera.get_single() {
        info!(
            "third person camera lock status: {:?}",
            third_person_camera.cursor_lock_active
        );
    }
}

pub(crate) fn setup_baseball_preview_scene(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut egui_user_textures: ResMut<EguiUserTextures>,
    selected_pitch_parameters: Res<SelectedPitchParameters>,
    mut query_baseball_preview: Query<&mut Transform, With<PreviewPassBaseballMarker>>,
) {
    let size = Extent3d {
        width: 280,
        height: 280,
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    // fill image.data with zeroes
    image.resize(size);

    let image_handle = images.add(image);
    egui_user_textures.add_image(image_handle.clone());
    commands.insert_resource(BaseballPreviewImage::new(image_handle.clone()));

    let render_layer = RenderLayers::from_layers(&[0]);

    if let Ok(mut transform) = query_baseball_preview.get_single_mut() {
        let seam_y_angle = selected_pitch_parameters.0.seam_y_angle;
        let seam_z_angle = selected_pitch_parameters.0.seam_z_angle;
        let rot =
            Quat::from_rotation_y(-seam_y_angle).mul_quat(Quat::from_rotation_z(seam_z_angle));
        *transform = transform.with_rotation(rot);
    }

    commands.spawn((
        PointLightBundle {
            transform: Transform::from_translation(Vec3::new(0.0, -10., 15.0)),
            ..default()
        },
        render_layer.clone(),
    ));

    // spawn preview pass camera
    commands.spawn((
        Camera3dBundle {
            projection: OrthographicProjection {
                scaling_mode: ScalingMode::FixedVertical(4.0),
                ..default()
            }
            .into(),
            camera: Camera {
                // render before the "main pass" camera
                order: -1,
                target: RenderTarget::Image(image_handle),
                clear_color: ClearColorConfig::Custom(Color::srgba(1.0, 1.0, 1.0, 0.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, -10., 15.0))
                .looking_at(Vec3::new(0.0, -10., 0.0), Vec3::Y),
            ..default()
        },
        render_layer.clone(),
    ));
}

pub(crate) fn setup_scene(mut commands: Commands) {
    // TODO: need to add render layers to blenvy
    commands.spawn((
        BlueprintInfo::from_path("levels/TheBullpen.glb"),
        SpawnBlueprint,
        HideUntilReady,
        GameWorldTag,
        RenderLayers::from_layers(&[0]),
    ));
}

#[derive(Debug, Component)]
pub(crate) struct BaseballMarker;

pub(crate) fn despawn_ball(
    mut commands: Commands,
    query_baseball: Query<Entity, With<BaseballMarker>>,
    active_batter_tracker: Res<ActiveBatterTracker>,
    mut ev_redraw: EventWriter<RedrawStrikezone>,
) {
    for baseball in query_baseball.iter() {
        commands.entity(baseball).despawn_recursive();
        ev_redraw.send(RedrawStrikezone {
            batter_height: active_batter_tracker.height,
        });
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
                    match selected_pitch_parameters.0.pitching_arm {
                        PitchingArm::Lefty => PITCH_DEFAULT_STARTING_POINT_LEFTY,
                        PitchingArm::Righty => PITCH_DEFAULT_STARTING_POINT_RIGHTY,
                    },
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
                Ccd::enabled(),
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
            let start_pos = match selected_pitch_parameters.0.pitching_arm {
                PitchingArm::Lefty => PITCH_DEFAULT_STARTING_POINT_LEFTY,
                PitchingArm::Righty => PITCH_DEFAULT_STARTING_POINT_RIGHTY,
            };
            let ray_dir = camera_transform.rotation.mul_vec3(-Vec3::Z).normalize();
            let max_toi = f32::INFINITY;
            let query = QueryFilter::new();

            let direction = match rapier_context.cast_ray(ray_origin, ray_dir, max_toi, true, query)
            {
                Some((_entity, toi)) => {
                    let aim_point = ray_origin + ray_dir * toi;
                    (aim_point - start_pos).normalize()
                }
                None => ray_dir,
            };

            selected_pitch_parameters.0.direction = direction;
        }

        let PitchParams {
            pitching_arm: _,
            gyro_pole,
            spin_efficiency,
            speed,
            spin_rate,
            tilt,
            direction,
            seam_y_angle,
            seam_z_angle,
        } = selected_pitch_parameters.0;

        let spin =
            get_angular_velocity_from_parameters(tilt, spin_efficiency, spin_rate, gyro_pole);

        velocity.linvel = direction * speed * 0.44704; // 0.3048;
        velocity.angvel = spin.from_baseball_coord_to_bevy();

        ev_activate_aerodynamics.send(ActivateAerodynamicsEvent {
            entity,
            seam_y_angle,
            seam_z_angle,
            //
            record_times: vec![],
            //
            strikezone_panels_z: (DEFAULT_FRONT_PANEL_POS_Z, DEFAULT_BACK_PANEL_POS_Z),
        });
    }
}

pub(crate) fn display_strikezone_panel_intersection_info(
    query_strikezone_panel: Query<(Entity, &StrikezonePanel)>,
    query_baseball_state: Query<&BaseballFlightState, With<BaseballMarker>>,
    mut collision_events: EventReader<CollisionEvent>,
    mut ev_record: EventWriter<RecordStrikezoneCollision>,
) {
    for collision_event in collision_events.read() {
        for (entity, panel) in query_strikezone_panel.iter() {
            match collision_event {
                // CollisionEvent::Started(collider1, collider2, _) => {}
                CollisionEvent::Stopped(collider1, collider2, event_flag) => {
                    if *event_flag == CollisionEventFlags::SENSOR {
                        let baseball_entity = if collider1.eq(&entity) {
                            Some(collider2)
                        } else if collider2.eq(&entity) {
                            Some(collider1)
                        } else {
                            None
                        };

                        if let Some(baseball_entity) = baseball_entity {
                            if let Ok(baseball_state) = query_baseball_state.get(*baseball_entity) {
                                let collision_point = match panel {
                                    StrikezonePanel::Front { .. } => {
                                        baseball_state.get_pos_at_strikezone_panels_z().0
                                    }
                                    StrikezonePanel::Back { .. } => {
                                        baseball_state.get_pos_at_strikezone_panels_z().1
                                    }
                                };
                                info!("collision point: {:?}", collision_point);
                                ev_record.send(RecordStrikezoneCollision {
                                    panel: entity,
                                    collision_point,
                                });
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
