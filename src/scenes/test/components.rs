use bevy::utils::HashMap;

use crate::prelude::*;

#[derive(Debug, Reflect, Clone, Component, Hash, PartialEq, Eq, PartialOrd)]
#[reflect(Component)]
pub(crate) enum BodyPartMarker {
    Pelvis,
    Torso,
    Shoulder,
    Elbow,
    Wrist,
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) enum PitchingArm {
    #[default]
    Left,
    Right,
}

impl PitchingArm {
    fn sign(&self) -> f32 {
        match self {
            PitchingArm::Left => 1.,
            PitchingArm::Right => -1.,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) enum PitchStage {
    #[default]
    // KneeUp
    Stride,
    ArmCocking,
    ArmAcceleration,
    ArmDeceleration,
    // FollowThrough
}

#[derive(Debug, Component, Clone)]
pub(crate) struct PitcherParams {
    pub height: f32,
    pub pitching_arm: PitchingArm,
    pub lateral_trunk_tilt: f32,
    //
    pub body_parts: HashMap<BodyPartMarker, Entity>,
}

impl Default for PitcherParams {
    fn default() -> Self {
        let body_parts = HashMap::<BodyPartMarker, Entity>::new();
        Self {
            height: 0.,
            pitching_arm: PitchingArm::default(),
            lateral_trunk_tilt: 0.,
            body_parts,
        }
    }
}

impl PitcherParams {
    pub(crate) fn build_core(&self, children: &mut ChildBuilder) -> Entity {
        children
            .spawn((
                RigidBody::KinematicPositionBased,
                GravityScale(0.),
                ColliderMassProperties::Density(100.),
                Collider::cuboid(0.1, 0.1, 0.1),
                TransformBundle::from_transform(Transform::from_translation(Vec3::new(0., 0., 0.))),
            ))
            .id()
    }

    pub(crate) fn build_pelvis(&self, core: Entity, children: &mut ChildBuilder) -> Entity {
        let ang_y_range: [f32; 2] = match self.pitching_arm {
            PitchingArm::Left => [-0.01, PI / 2. + 0.01],
            PitchingArm::Right => [-PI / 2. - 0.01, 0.01],
        };

        let pelvic_joint = GenericJointBuilder::new(JointAxesMask::LIN_AXES)
            .local_anchor1(Vec3::new(0., 1.0, 0.0))
            .local_anchor2(Vec3::new(0., 0.0, 0.0))
            .coupled_axes(JointAxesMask::LIN_AXES)
            .motor_position(
                JointAxis::AngY,
                self.pitching_arm.sign() * PI / 2.,
                1.,
                0.01,
            )
            .motor_model(JointAxis::AngY, MotorModel::ForceBased)
            .limits(JointAxis::AngX, [-0., 0.])
            .limits(JointAxis::AngY, ang_y_range)
            .limits(JointAxis::AngZ, [-0., 0.])
            .build();

        children
            .spawn((
                RigidBody::Dynamic,
                GravityScale(0.),
                Collider::cuboid(0.3, 0.1, 0.1),
                TransformBundle::from_transform(Transform::from_translation(Vec3::new(0., 1., 0.))),
                ImpulseJoint::new(core, TypedJoint::GenericJoint(pelvic_joint)),
                BodyPartMarker::Pelvis,
            ))
            .id()
    }

    pub(crate) fn build_upper_torso(&self, pelvis: Entity, children: &mut ChildBuilder) -> Entity {
        let spinal_joint = GenericJointBuilder::new(JointAxesMask::LIN_AXES)
            .local_anchor1(Vec3::new(0., 0.6, 0.0))
            .local_anchor2(Vec3::new(0., 0.0, 0.0))
            .coupled_axes(JointAxesMask::LIN_AXES)
            // .motor_position(JointAxis::AngY, 0., 1., 0.01)
            .motor_model(JointAxis::AngY, MotorModel::ForceBased)
            .limits(JointAxis::AngX, [-0., 0.])
            .limits(JointAxis::AngY, [-0.1, 0.1])
            .limits(JointAxis::AngZ, [-0., 0.])
            .build();

        children
            .spawn((
                RigidBody::Dynamic,
                GravityScale(0.),
                Collider::cuboid(0.5, 0.1, 0.1),
                ColliderMassProperties::Density(10.0),
                TransformBundle::from_transform(Transform::from_translation(Vec3::new(
                    0., 1.6, 0.,
                ))),
                ImpulseJoint::new(pelvis, TypedJoint::GenericJoint(spinal_joint)),
                BodyPartMarker::Torso,
            ))
            .id()
    }

    pub(crate) fn build_shoulder(
        &self,
        upper_torso: Entity,
        children: &mut ChildBuilder,
    ) -> Entity {
        let shoulder_joint = GenericJointBuilder::new(JointAxesMask::LIN_AXES)
            .local_anchor1(Vec3::new(self.pitching_arm.sign() * 0.8, 0.0, 0.0))
            .local_anchor2(Vec3::new(0., 0.0, 0.0))
            .coupled_axes(JointAxesMask::LIN_AXES)
            // .motor_position(JointAxis::AngX, 0., 0.1, 0.01)
            // .motor_model(JointAxis::AngX, MotorModel::ForceBased)
            .limits(JointAxis::AngX, [-0., 0.])
            .limits(JointAxis::AngY, [-0., 0.])
            .limits(JointAxis::AngZ, [-0., 0.])
            .build();

        children
            .spawn((
                RigidBody::Dynamic,
                GravityScale(0.),
                Collider::cuboid(0.05, 0.05, 0.05),
                ColliderMassProperties::Density(10.0),
                TransformBundle::from_transform(Transform::from_translation(Vec3::new(
                    self.pitching_arm.sign() * 2.,
                    1.6,
                    0.,
                ))),
                ImpulseJoint::new(upper_torso, TypedJoint::GenericJoint(shoulder_joint)),
                BodyPartMarker::Shoulder,
            ))
            .id()
    }

    pub(crate) fn build_elbow(&self, shoulder: Entity, children: &mut ChildBuilder) -> Entity {
        let ang_z_range: [f32; 2] = match self.pitching_arm {
            PitchingArm::Left => [-0.01, PI / 2. + 0.01],
            PitchingArm::Right => [-PI / 2. - 0.01, 0.01],
        };

        let elbow_joint = GenericJointBuilder::new(JointAxesMask::LIN_AXES)
            .local_anchor1(Vec3::new(self.pitching_arm.sign() * 0.8, 0.0, 0.0))
            .local_anchor2(Vec3::new(0., 0.0, 0.0))
            .coupled_axes(JointAxesMask::LIN_AXES)
            .motor_position(
                JointAxis::AngZ,
                self.pitching_arm.sign() * PI / 2.,
                1.,
                0.01,
            )
            .motor_model(JointAxis::AngZ, MotorModel::ForceBased)
            .limits(JointAxis::AngX, [-0., 0.])
            .limits(JointAxis::AngY, [-0., 0.])
            .limits(JointAxis::AngZ, ang_z_range)
            .build();

        children
            .spawn((
                RigidBody::Dynamic,
                GravityScale(0.),
                Collider::cuboid(0.05, 0.05, 0.05),
                ColliderMassProperties::Density(10.0),
                TransformBundle::from_transform(Transform::from_translation(Vec3::new(
                    self.pitching_arm.sign() * 3.,
                    1.6,
                    0.,
                ))),
                ImpulseJoint::new(shoulder, TypedJoint::GenericJoint(elbow_joint)),
                BodyPartMarker::Elbow,
            ))
            .id()
    }

    pub(crate) fn build_wrist(&self, elbow: Entity, children: &mut ChildBuilder) -> Entity {
        let wrist_joint = GenericJointBuilder::new(JointAxesMask::LIN_AXES)
            .local_anchor1(Vec3::new(self.pitching_arm.sign() * 0.8, 0.0, 0.0))
            .local_anchor2(Vec3::new(0., 0.0, 0.0))
            .coupled_axes(JointAxesMask::LIN_AXES)
            .motor_position(JointAxis::AngZ, 0., 1., 0.01)
            .motor_model(JointAxis::AngZ, MotorModel::ForceBased)
            .limits(JointAxis::AngX, [-0.01, 0.01])
            .limits(JointAxis::AngY, [-0.01, 0.01])
            .limits(JointAxis::AngZ, [-0.01, 0.01])
            .build();

        children
            .spawn((
                RigidBody::Dynamic,
                GravityScale(0.),
                Collider::cuboid(0.05, 0.05, 0.05),
                ColliderMassProperties::Density(10.0),
                TransformBundle::from_transform(Transform::from_translation(Vec3::new(
                    self.pitching_arm.sign() * 4.,
                    1.6,
                    0.,
                ))),
                ImpulseJoint::new(elbow, TypedJoint::GenericJoint(wrist_joint)),
                Velocity::default(),
                BodyPartMarker::Wrist,
            ))
            .id()
    }

    pub(crate) fn on_foot_contact(
        &self,
        commands: &mut Commands,
        entity: Entity,
        impulse_joint: &mut ImpulseJoint,
        body_part: &mut BodyPartMarker,
    ) {
        match body_part {
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
                    .motor_position(JointAxis::AngZ, self.lateral_trunk_tilt, 1., 0.01)
                    .motor_model(JointAxis::AngZ, MotorModel::ForceBased)
                    .limits(JointAxis::AngX, [-0., 0.])
                    .limits(JointAxis::AngY, [-0.1, 0.1])
                    .limits(JointAxis::AngZ, [-0., self.lateral_trunk_tilt])
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
