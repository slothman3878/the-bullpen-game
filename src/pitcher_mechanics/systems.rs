use crate::prelude::*;

pub(crate) fn spawn_pitcher_mechanics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let starting_pos = Vec3::new(0., 0., 18.44); // Vec3::ZERO;
    let pitcher = commands
        .spawn((
            TransformBundle::from_transform(Transform::from_translation(starting_pos)),
            PitchStage::default(),
        ))
        .id();
    // let direction = Vec3::Z;
    let rotation: f32 = PI;

    let mut params = PitcherParams {
        pitching_arm: PitchingArm::Right,
        lateral_trunk_tilt: 45. * PI / 180.,
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