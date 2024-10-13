use crate::prelude::*;

// this is at pevlis break
pub(crate) fn mark_velo(mut query_arm_part: Query<(&BodyPartMarker, &Velocity)>) {
    for (body_part, velo) in query_arm_part.iter_mut() {
        match body_part {
            BodyPartMarker::Wrist => {
                info!("velo: {:?}", velo.linvel.length());
            }
            _ => {}
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
        lateral_trunk_tilt: 70. * PI / 180.,
        rotation: Quat::from_rotation_y(0.),
        ..default()
    };
    commands
        .spawn((
            TransformBundle::from_transform(Transform::from_translation(Vec3::ZERO)),
            PitchStage::default(),
        ))
        .with_children(|children| {
            let core = params.build_core(children);

            let pelvis = params.build_pelvis(core, children);
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

            let upper_torso = params.build_upper_torso(pelvis, children);
            params.body_parts.insert(BodyPartMarker::Torso, upper_torso);

            let shoulder = params.build_shoulder(upper_torso, children);
            params.body_parts.insert(BodyPartMarker::Shoulder, shoulder);

            // // arm deceleration trigger for shoulder
            // let arm_dec_trigger = children
            //     .spawn((
            //         Sensor,
            //         Collider::cuboid(0.001, 0.1, 0.001),
            //         TransformBundle::from_transform(Transform::from_translation(Vec3::new(
            //             0.2, // apply pitching arm sign
            //             1.6, 0.08,
            //         ))),
            //     ))
            //     .id();
            // params.arm_deceleration_trigger = Some(arm_dec_trigger);

            let elbow = params.build_elbow(shoulder, children);
            params.body_parts.insert(BodyPartMarker::Elbow, elbow);

            let wrist = params.build_wrist(elbow, children, &mut meshes, &mut materials);
            params.body_parts.insert(BodyPartMarker::Wrist, wrist);
        })
        .insert(params);
}
