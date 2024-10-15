use crate::prelude::*;

// this is at pevlis break
pub(crate) fn mark_velo(
    query_pitch_stage: Query<(&PitcherParams, &PitchStage)>,
    query_ball: Query<&Velocity, With<TempBallMarker>>,
) {
    for (_pitcher_params, pitch_stage) in query_pitch_stage.iter() {
        if *pitch_stage < PitchStage::ArmAcceleration {
            return;
        }
        for velo in query_ball.iter() {
            info!(
                "velo: {:?}, direction: {:?}",
                velo.linvel.length(),
                velo.linvel.normalize()
            );
        }
    }
}

pub(crate) fn spawn_arms(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut params = PitcherParams {
        height: 1.85,
        pitching_arm: PitchingArm::Left,
        lateral_trunk_tilt: 30. * PI / 180.,
        rotation: Quat::from_rotation_y(0.),
        ..default()
    };
    let pitcher = commands
        .spawn((
            TransformBundle::from_transform(Transform::from_translation(Vec3::ZERO)),
            PitchStage::default(),
        ))
        .id();

    // should not make this children
    let core_transform = Transform::from_translation(Vec3::new(0., 0.5, 0.));
    let core = params.build_core(&mut commands, core_transform);
    params.body_parts.insert(BodyPartMarker::Core, core);

    commands.entity(core).with_children(|children| {
        // pelvic sensor
        let pelvic_break = children
            .spawn((
                Sensor,
                Collider::cuboid(0.001, 0.1, 0.001),
                TransformBundle::from_transform(Transform::from_translation(Vec3::new(
                    params.pitching_arm.sign() * 0.2, // apply pitching arm sign
                    0.5,
                    0.,
                ))),
            ))
            .id();
        params.pelvic_break = Some(pelvic_break);
    });

    let pelvic_transform =
        Transform::from_translation(core_transform.translation + Vec3::new(0., 0.5, 0.))
            .with_rotation(Quat::from_rotation_y(params.pitching_arm.sign() * PI / 2.));
    let pelvis = params.build_pelvis(core, &mut commands, pelvic_transform);
    params.body_parts.insert(BodyPartMarker::Pelvis, pelvis);

    let torso_transform =
        Transform::from_translation(pelvic_transform.translation + Vec3::new(0., 0.6, 0.))
            .with_rotation(Quat::from_rotation_y(params.pitching_arm.sign() * PI / 2.));
    let upper_torso = params.build_upper_torso(pelvis, &mut commands, torso_transform);
    params.body_parts.insert(BodyPartMarker::Torso, upper_torso);

    // need to consider the vector from elbow to torso
    let shoulder_translation = torso_transform.translation + Vec3::new(0., 0., -0.4);
    // the following two need to an input
    let elbow_translation = shoulder_translation + Vec3::new(0., 0., -0.3);
    let wrist_translation = elbow_translation + Vec3::new(0., 0., -0.3);

    let shoulder_transform = Transform::from_translation(shoulder_translation)
        .with_rotation(Quat::from_rotation_y(params.pitching_arm.sign() * PI / 2.));
    let shoulder = params.build_shoulder(upper_torso, &mut commands, shoulder_transform);
    params.body_parts.insert(BodyPartMarker::Shoulder, shoulder);

    let elbow_transform = Transform::from_translation(elbow_translation)
        .with_rotation(Quat::from_rotation_y(params.pitching_arm.sign() * PI / 2.));
    let elbow = params.build_elbow(shoulder, &mut commands, elbow_transform);
    params.body_parts.insert(BodyPartMarker::Elbow, elbow);

    let wrist_transform = Transform::from_translation(wrist_translation)
        .with_rotation(Quat::from_rotation_y(params.pitching_arm.sign() * PI / 2.));
    let wrist = params.build_wrist(elbow, &mut commands, wrist_transform);
    params.body_parts.insert(BodyPartMarker::Wrist, wrist);

    let ball_transfomr = Transform::from_translation(wrist_translation + Vec3::new(0., 0., -0.05))
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
