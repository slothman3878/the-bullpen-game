use crate::prelude::*;

const STARTING_POSITION: Vec3 = Vec3::new(0., 0., 18.44);

pub(crate) fn spawn_pitcher(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((
            TransformBundle::from_transform(Transform::from_translation(STARTING_POSITION)),
            PitchStage::default(),
            PitcherMarker,
        ))
        .with_children(|children| {
            children.spawn((
                ThirdPersonCameraTarget,
                TransformBundle::from_transform(Transform::from_translation(
                    STARTING_POSITION + Vec3::new(0., 1.6, 0.),
                )),
            ));
        });

    commands
        .spawn((
            ThirdPersonCamera {
                aim_speed: 5.0,
                cursor_lock_toggle_enabled: true,
                offset_enabled: true,
                offset: Offset::new(0.5, 0.),
                zoom: Zoom::new(8., 20.),
                cursor_lock_key: KeyCode::Escape,
                ..default()
            },
            Camera3dBundle {
                projection: blenvy::Projection::Perspective(PerspectiveProjection {
                    fov: 10.0_f32.to_radians(),
                    ..default()
                }),
                camera: Camera {
                    is_active: true,
                    order: 2,
                    ..default()
                },
                ..default()
            },
            PitcherCameraMarker,
            Name::new("pitcher camera"),
            InheritedVisibility::VISIBLE,
        ))
        .with_children(|parent| {
            parent.spawn((PbrBundle {
                mesh: meshes.add(Sphere::new(0.0005)).into(), // default 0.075
                material: materials.add(Color::srgb(0.1, 0.1, 0.1)),
                transform: Transform::from_xyz(0., 0., -0.5),
                ..default()
            },));
        });
}

pub(crate) fn spawn_pitcher_mechanics(
    query_pitcher: Query<Entity, With<PitcherMarker>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for pitcher in query_pitcher.iter() {
        let starting_pos = STARTING_POSITION;
        // let direction = Vec3::Z;
        let rotation: f32 = PI;

        let mut params = PitcherParams {
            pitching_arm: PitchingArm::Left,
            lateral_trunk_tilt: 30. * PI / 180.,
            direction: Quat::from_rotation_y(rotation).mul_vec3(Vec3::Z),
            starting_pos,
            ..default()
        };
        let distance_from_ground: f32 = DISTANCE_CORE_HIP - 0.1;

        let balance_weight_transform = Transform::from_translation(
            starting_pos + Vec3::new(0., params.leg_length - params.torso_drop, 0.),
        )
        .with_rotation(Quat::from_rotation_y(rotation));
        let balance_weight = params.build_balance_weight(&mut commands, balance_weight_transform);
        params.balance_weight = Some(balance_weight);

        // should not make this children
        let core_transform = Transform::from_translation(
            starting_pos
                + Vec3::new(0., params.leg_length, 0.)
                + Vec3::new(0., distance_from_ground, 0.),
        )
        .with_rotation(Quat::from_rotation_y(rotation));
        let core = params.build_core(balance_weight, &mut commands, core_transform);
        params.body_parts.insert(PitcherBodyPartMarker::Core, core);

        let back_hip_transform = Transform::from_translation(
            core_transform.translation - Vec3::new(0., DISTANCE_CORE_HIP, 0.),
        )
        .with_rotation(Quat::from_rotation_y(rotation));
        let back_hip = params.build_back_hip(core, &mut commands, back_hip_transform);
        params
            .body_parts
            .insert(PitcherBodyPartMarker::BackHip, back_hip);

        let back_ankle_transform = Transform::from_translation(
            back_hip_transform.translation - Vec3::new(0., params.leg_length, 0.),
        )
        .with_rotation(Quat::from_rotation_y(rotation));
        let back_ankle = params.build_back_ankle(back_hip, &mut commands, back_ankle_transform);
        params
            .body_parts
            .insert(PitcherBodyPartMarker::BackFoot, back_ankle);

        commands.entity(core).with_children(|children| {
            // pelvic sensor
            let pelvic_break = children
                .spawn((
                    Sensor,
                    Collider::cuboid(0.001, 0.1, 0.001),
                    TransformBundle::from_transform(Transform::from_translation(Vec3::new(
                        params.pitching_arm.sign() * 0.15, // apply pitching arm sign
                        params.waist_length,
                        0.,
                    ))),
                ))
                .id();
            params.pelvic_break = Some(pelvic_break);
        });

        let pelvic_transform = Transform::from_translation(
            core_transform.translation + Vec3::new(0., params.waist_length, 0.),
        )
        .with_rotation(Quat::from_rotation_y(params.pitching_arm.sign() * PI / 2.));
        let pelvis = params.build_pelvis(core, &mut commands, pelvic_transform);
        params
            .body_parts
            .insert(PitcherBodyPartMarker::Pelvis, pelvis);

        let torso_transform = Transform::from_translation(
            pelvic_transform.translation + Vec3::new(0., params.chest_length, 0.),
        )
        .with_rotation(Quat::from_rotation_y(params.pitching_arm.sign() * PI / 2.));
        let upper_torso = params.build_upper_torso(pelvis, &mut commands, torso_transform);
        params
            .body_parts
            .insert(PitcherBodyPartMarker::Torso, upper_torso);

        // need to consider the vector from elbow to torso
        let shoulder_translation =
            torso_transform.translation + Vec3::new(0., 0., -DISTANCE_CHEST_SHOULDER);
        // the following two need to an input
        let elbow_translation = shoulder_translation + Vec3::new(0., 0., -params.upper_arm_length);
        let wrist_translation = elbow_translation + Vec3::new(0., 0., -params.forearm_length);

        let shoulder_transform = Transform::from_translation(shoulder_translation)
            .with_rotation(Quat::from_rotation_y(params.pitching_arm.sign() * PI / 2.));
        let shoulder = params.build_shoulder(upper_torso, &mut commands, shoulder_transform);
        params
            .body_parts
            .insert(PitcherBodyPartMarker::Shoulder, shoulder);

        let elbow_transform = Transform::from_translation(elbow_translation)
            .with_rotation(Quat::from_rotation_y(params.pitching_arm.sign() * PI / 2.));
        let elbow = params.build_elbow(shoulder, &mut commands, elbow_transform);
        params
            .body_parts
            .insert(PitcherBodyPartMarker::Elbow, elbow);

        let wrist_transform = Transform::from_translation(wrist_translation)
            .with_rotation(Quat::from_rotation_y(params.pitching_arm.sign() * PI / 2.));
        let wrist = params.build_wrist(elbow, &mut commands, wrist_transform);
        params
            .body_parts
            .insert(PitcherBodyPartMarker::Wrist, wrist);

        let ball_transfomr =
            Transform::from_translation(wrist_translation + Vec3::new(0., 0., -0.05))
                .with_rotation(Quat::from_rotation_y(params.pitching_arm.sign() * PI / 2.));
        let ball = params.build_ball(
            wrist,
            &mut commands,
            &mut meshes,
            &mut materials,
            ball_transfomr,
        );
        params.ball = Some(ball);

        commands.entity(pitcher).insert(params);
    }
}

pub(crate) fn core_position_tracker(
    mut ev_pitch_stage_transition_event: EventWriter<PitchStageTransitionEvents>,
    query_global_transform: Query<&Transform, With<PitcherBodyPartMarker>>,
    mut query_pitcher: Query<(Entity, &PitcherParams, &mut PitchStage)>, // there must only one pitcher at a time?
) {
    for (entity, pitcher_params, mut pitch_stage) in query_pitcher.iter_mut() {
        if *pitch_stage != PitchStage::Stride {
            return;
        }
        if let Some(core_entity) = pitcher_params.body_parts.get(&PitcherBodyPartMarker::Core) {
            if let Ok(transform) = query_global_transform.get(*core_entity) {
                if transform.translation.y < (pitcher_params.leg_length - pitcher_params.torso_drop)
                {
                    ev_pitch_stage_transition_event
                        .send(PitchStageTransitionEvents::FootContact(entity));
                    *pitch_stage = PitchStage::ArmCocking;
                    info!("pitcher event: transitioning to arm cocking");
                }
            }
        }
    }
}

pub(crate) fn pelvic_rotation_tracker(
    mut ev_pitch_stage_transition_event: EventWriter<PitchStageTransitionEvents>,
    rapier_context: Res<RapierContext>,
    mut query_pitcher: Query<(Entity, &PitcherParams, &mut PitchStage)>,
) {
    for (entity, pitcher_params, mut pitch_stage) in query_pitcher.iter_mut() {
        if *pitch_stage != PitchStage::ArmCocking {
            return;
        }
        if let (Some(pelvis), Some(pelvic_break_sensor)) = (
            pitcher_params
                .body_parts
                .get(&PitcherBodyPartMarker::Pelvis),
            pitcher_params.pelvic_break,
        ) {
            if Some(true) == rapier_context.intersection_pair(*pelvis, pelvic_break_sensor) {
                ev_pitch_stage_transition_event.send(PitchStageTransitionEvents::MaxER(entity));
                *pitch_stage = PitchStage::ArmAcceleration;
                info!("pitcher event: transitioning to arm acceleration");
            }
        }
    }
}

pub(crate) fn wrist_z_pos_tracker(
    mut ev_pitch_stage_transition_event: EventWriter<PitchStageTransitionEvents>,
    query_transform: Query<&Transform, With<PitcherBodyPartMarker>>,
    mut query_pitcher: Query<(Entity, &PitcherParams, &mut PitchStage)>,
) {
    for (entity, pitcher_params, mut pitch_stage) in query_pitcher.iter_mut() {
        if *pitch_stage != PitchStage::ArmAcceleration {
            return;
        }
        if let Some(wrist) = pitcher_params.body_parts.get(&PitcherBodyPartMarker::Wrist) {
            if let Ok(transform) = query_transform.get(*wrist) {
                let rotation = pitcher_params.direction.angle_between(Vec3::Z);
                if Quat::from_rotation_y(-rotation)
                    .mul_vec3(transform.translation - pitcher_params.starting_pos)
                    .z
                    > 0.
                {
                    ev_pitch_stage_transition_event.send(PitchStageTransitionEvents::MaxIR(entity));
                    *pitch_stage = PitchStage::ArmDeceleration;
                    info!("pitcher event: transitioning to arm deceleration");
                }
            }
        }
    }
}
