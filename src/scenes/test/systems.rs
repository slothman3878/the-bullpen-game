use crate::prelude::*;

// this is at pevlis break
pub(crate) fn mark_velo(
    query_pitch_stage: Query<(&PitcherParams, &PitchStage)>,
    query_arm_part: Query<(&BodyPartMarker, &Velocity)>,
) {
    for (pitcher_params, pitch_stage) in query_pitch_stage.iter() {
        if *pitch_stage < PitchStage::ArmAcceleration {
            return;
        }
        if let Some(wrist) = pitcher_params.body_parts.get(&BodyPartMarker::Wrist) {
            if let Ok((body_part, velo)) = query_arm_part.get(*wrist) {
                match body_part {
                    BodyPartMarker::Wrist => {
                        info!(
                            "velo: {:?}, direction: {:?}",
                            velo.linvel.length(),
                            velo.linvel.normalize()
                        );
                    }
                    _ => {}
                }
            }
        }
    }
}

pub(crate) fn apply_force_shoulder(
    query_pitcher: Query<(&PitcherParams, &PitchStage)>,
    mut query_arm_part: Query<(&BodyPartMarker, &Transform, &mut ExternalForce)>,
) {
    // for (pitcher_params, pitch_stage) in query_pitcher.iter() {
    //     if let Some(shoulder) = pitcher_params.body_parts.get(&BodyPartMarker::Shoulder) {
    //         if let Ok((body_part, transform, mut external_force)) =
    //             query_arm_part.get_mut(*shoulder)
    //         {
    //             match body_part {
    //                 BodyPartMarker::Shoulder => {
    //                     if *pitch_stage > PitchStage::ArmAcceleration {
    //                         *external_force = ExternalForce::at_point(
    //                             1. * transform.rotation.mul_vec3(Vec3::Y).normalize(),
    //                             transform.translation,
    //                             transform.translation,
    //                         )
    //                     } else {
    //                         *external_force = ExternalForce::default();
    //                     }
    //                 }
    //                 _ => {}
    //             }
    //         }
    //     }
    // }
}

pub(crate) fn spawn_arms(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut params = PitcherParams {
        height: 1.85,
        pitching_arm: PitchingArm::Left,
        lateral_trunk_tilt: 50. * PI / 180.,
        rotation: Quat::from_rotation_y(0.),
        ..default()
    };
    commands
        .spawn((
            TransformBundle::from_transform(Transform::from_translation(Vec3::ZERO)),
            PitchStage::default(),
        ))
        .with_children(|children| {
            // let ball = params.build_ball(wrist, children, &mut meshes, &mut materials);
        })
        .insert(params);

    // should not make this children
    let core_transform = Transform::from_translation(Vec3::new(0., 0., 0.));
    let core = params.build_core(children, core_transform);

    let pelvic_transform =
        Transform::from_translation(core_transform.translation + Vec3::new(0., 1., 0.))
            .with_rotation(Quat::from_rotation_y(PI / 2.));
    let pelvis = params.build_pelvis(core, children, pelvic_transform);
    params.body_parts.insert(BodyPartMarker::Pelvis, pelvis);

    // pelvic sensor
    let pelvic_break = children
        .spawn((
            Sensor,
            Collider::cuboid(0.001, 0.1, 0.001),
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(
                0.2, // apply pitching arm sign
                1., 0.08,
            ))),
        ))
        .id();
    params.pelvic_break = Some(pelvic_break);

    let torso_transform =
        Transform::from_translation(pelvic_transform.translation + Vec3::new(0., 0.6, 0.))
            .with_rotation(Quat::from_rotation_y(PI / 2.));
    let upper_torso = params.build_upper_torso(pelvis, children, torso_transform);
    params.body_parts.insert(BodyPartMarker::Torso, upper_torso);

    // need to consider the vector from elbow to torso
    let shoulder_translation = torso_transform.translation + Vec3::new(0., 0., -0.4);
    // the following two need to an input
    let elbow_translation = shoulder_translation + Vec3::new(0., 0., -0.3);
    let wrist_translation = elbow_translation + Vec3::new(0., 0., -0.3);

    let shoulder_transform = Transform::from_translation(shoulder_translation)
        .with_rotation(Quat::from_rotation_y(PI / 2.));
    let shoulder =
        params.build_shoulder(upper_torso, children, torso_transform, shoulder_transform);
    params.body_parts.insert(BodyPartMarker::Shoulder, shoulder);

    let elbow_transform = Transform::from_translation(elbow_translation)
        .with_rotation(Quat::from_rotation_y(PI / 2.));
    let elbow = params.build_elbow(shoulder, children, shoulder_transform, elbow_transform);
    params.body_parts.insert(BodyPartMarker::Elbow, elbow);

    let wrist_transform = Transform::from_translation(wrist_translation)
        .with_rotation(Quat::from_rotation_y(PI / 2.));
    let wrist = params.build_wrist(elbow, children, elbow_transform, wrist_transform);
    params.body_parts.insert(BodyPartMarker::Wrist, wrist);
}
