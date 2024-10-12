use crate::prelude::*;

// this is at foot contact
pub(crate) fn max_er(
    mut commands: Commands,
    mut query_arm_part: Query<(Entity, &mut ImpulseJoint, &BodyPartMarker)>,
) {
    for (entity, mut impulse_joint, arm_part) in query_arm_part.iter_mut() {
        match arm_part {
            BodyPartMarker::Pelvis => {
                let new_joint = GenericJointBuilder::new(JointAxesMask::LIN_AXES)
                    .local_anchor1(Vec3::new(0., 1.0, 0.0))
                    .local_anchor2(Vec3::new(0., 0.0, 0.0))
                    .coupled_axes(JointAxesMask::LIN_AXES)
                    // .motor_position(JointAxis::AngY, 0., 1., 0.0001)
                    // .motor_model(JointAxis::AngY, MotorModel::ForceBased)
                    .limits(JointAxis::AngX, [-0., 0.])
                    .limits(JointAxis::AngY, [-0.01, PI / 2. + 0.01])
                    .limits(JointAxis::AngZ, [-0., 0.])
                    .build();

                impulse_joint.data = TypedJoint::GenericJoint(new_joint);

                commands.entity(entity).insert(ExternalImpulse::at_point(
                    1. * Vec3::X,
                    Vec3::new(0., 1., -0.2),
                    Vec3::new(0., 1., 0.),
                ));
            }
            BodyPartMarker::Torso => {
                let new_joint = GenericJointBuilder::new(JointAxesMask::LIN_AXES)
                    .local_anchor1(Vec3::new(0., 0.6, 0.0))
                    .local_anchor2(Vec3::new(0., 0.0, 0.0))
                    .coupled_axes(JointAxesMask::LIN_AXES)
                    .motor_position(JointAxis::AngZ, PI / 4., 1., 0.01)
                    .motor_model(JointAxis::AngZ, MotorModel::ForceBased)
                    .limits(JointAxis::AngX, [-0., 0.])
                    .limits(JointAxis::AngY, [-0.1, 0.1])
                    .limits(JointAxis::AngZ, [-0., PI / 4.])
                    .build();

                impulse_joint.data = TypedJoint::GenericJoint(new_joint);
            }
            BodyPartMarker::Shoulder => {
                let new_joint = GenericJointBuilder::new(JointAxesMask::LIN_AXES)
                    .local_anchor1(Vec3::new(0.8, 0.0, 0.0))
                    .local_anchor2(Vec3::new(0., 0.0, 0.0))
                    .coupled_axes(JointAxesMask::LIN_AXES)
                    .motor_position(JointAxis::AngX, -PI / 2., 0.9, 0.1)
                    .motor_model(JointAxis::AngX, MotorModel::ForceBased)
                    // .limits(JointAxis::AngX, [-0.01, PI / 2. + 0.01])
                    .limits(JointAxis::AngY, [-0., 0.])
                    .limits(JointAxis::AngZ, [-0., 0.])
                    .build();
                impulse_joint.data = TypedJoint::GenericJoint(new_joint);
            }
            _ => {}
        }
    }
}

// this is at pevlis break
pub(crate) fn release(
    mut query_arm_part: Query<(Entity, &mut ImpulseJoint, &BodyPartMarker, &Velocity)>,
) {
    for (_, mut impulse_joint, arm_part, velo) in query_arm_part.iter_mut() {
        match arm_part {
            BodyPartMarker::Pelvis => {
                let new_joint = GenericJointBuilder::new(JointAxesMask::LIN_AXES)
                    .local_anchor1(Vec3::new(0., 1.0, 0.0))
                    .local_anchor2(Vec3::new(0., 0.0, 0.0))
                    .coupled_axes(JointAxesMask::LIN_AXES)
                    // .motor_position(JointAxis::AngY, 0., 1., 0.0001)
                    // .motor_model(JointAxis::AngY, MotorModel::ForceBased)
                    .limits(JointAxis::AngX, [-0., 0.])
                    .limits(JointAxis::AngY, [-0.01, 0.01])
                    .limits(JointAxis::AngZ, [-0., 0.])
                    .build();

                impulse_joint.data = TypedJoint::GenericJoint(new_joint);
            }
            BodyPartMarker::Torso => {
                let new_joint = GenericJointBuilder::new(JointAxesMask::LIN_AXES)
                    .local_anchor1(Vec3::new(0., 0.6, 0.0))
                    .local_anchor2(Vec3::new(0., 0.0, 0.0))
                    .coupled_axes(JointAxesMask::LIN_AXES)
                    .motor_position(JointAxis::AngZ, PI / 4., 1., 0.01)
                    .motor_model(JointAxis::AngZ, MotorModel::ForceBased)
                    .limits(JointAxis::AngX, [-0., 0.])
                    .limits(JointAxis::AngY, [-PI / 4. - 0.01, PI / 2. + 0.01])
                    .limits(JointAxis::AngZ, [-0., PI / 4.])
                    .build();

                impulse_joint.data = TypedJoint::GenericJoint(new_joint);
            }
            BodyPartMarker::Elbow => {
                let new_joint = GenericJointBuilder::new(JointAxesMask::LIN_AXES)
                    .local_anchor1(Vec3::new(0.8, 0.0, 0.0))
                    .local_anchor2(Vec3::new(0., 0.0, 0.0))
                    .coupled_axes(JointAxesMask::LIN_AXES)
                    .motor_position(JointAxis::AngZ, 0., 0.9, 0.1)
                    .motor_position(JointAxis::AngY, 0., 0.9, 0.1)
                    .motor_model(JointAxis::AngZ, MotorModel::ForceBased)
                    .motor_model(JointAxis::AngY, MotorModel::ForceBased)
                    .limits(JointAxis::AngX, [-0., 0.])
                    .limits(JointAxis::AngY, [-0.01, PI / 2. + 0.01])
                    .limits(JointAxis::AngZ, [-0.01, PI / 2. + 0.01])
                    .build();

                impulse_joint.data = TypedJoint::GenericJoint(new_joint);
            }
            BodyPartMarker::Shoulder => {
                let new_joint = GenericJointBuilder::new(JointAxesMask::LIN_AXES)
                    .local_anchor1(Vec3::new(0.8, 0.0, 0.0))
                    .local_anchor2(Vec3::new(0., 0.0, 0.0))
                    .coupled_axes(JointAxesMask::LIN_AXES)
                    .motor_position(JointAxis::AngX, 0., 0.9, 0.1)
                    .motor_model(JointAxis::AngX, MotorModel::ForceBased)
                    // .limits(JointAxis::AngX, [-0.01, PI / 2. + 0.01])
                    .limits(JointAxis::AngY, [-0., 0.])
                    .limits(JointAxis::AngZ, [-0., 0.])
                    .build();
                impulse_joint.data = TypedJoint::GenericJoint(new_joint);
            }
            BodyPartMarker::Wrist => {
                info!("velo: {:?}", velo.linvel.length());
            }
        }
    }
}

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

pub(crate) fn spawn_arms(mut commands: Commands) {
    let params = PitcherParams {
        height: 1.85,
        pitching_arm: PitchingArm::Left,
        lateral_trunk_tilt: PI / 4.,
    };
    commands
        .spawn((
            params.clone(),
            TransformBundle::from_transform(Transform::from_translation(Vec3::ZERO)),
        ))
        .with_children(|children| {
            let core = params.build_core(children);

            let pelvis = params.build_pelvis(core, children);

            let upper_torso = params.build_upper_torso(pelvis, children);

            let shoulder = params.build_shoulder(upper_torso, children);

            let elbow = params.build_elbow(shoulder, children);

            let wrist = params.build_wrist(elbow, children);
        });
}
