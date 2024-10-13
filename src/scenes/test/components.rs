use bevy::utils::HashMap;

use crate::prelude::*;

#[derive(Debug, Reflect, Clone, Component)]
#[reflect(Component)]
pub(crate) struct PelvicBreakTriggerMarker;

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

#[derive(Debug, Component, Reflect, Clone, Copy, Default, PartialEq, Eq)]
#[reflect(Component)]
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
    pub height: f32, // individual proportions instead
    pub pitching_arm: PitchingArm,
    pub lateral_trunk_tilt: f32, // PI / 2. - lateral_trunk_tilt
    pub rotation: Quat,
    //
    pub body_parts: HashMap<BodyPartMarker, Entity>,
    // various triggers
    pub pelvic_break: Option<Entity>,
    pub arm_deceleration_trigger: Option<Entity>,
}

impl Default for PitcherParams {
    fn default() -> Self {
        let body_parts = HashMap::<BodyPartMarker, Entity>::new();
        Self {
            height: 0.,
            pitching_arm: PitchingArm::default(),
            lateral_trunk_tilt: PI / 2.,
            rotation: Quat::from_rotation_y(0.),
            body_parts,
            pelvic_break: None,
            arm_deceleration_trigger: None,
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
        // let ang_y_range: [f32; 2] = match self.pitching_arm {
        //     PitchingArm::Left => [-0.01, PI / 2. + 0.01],
        //     PitchingArm::Right => [-PI / 2. - 0.01, 0.01],
        // };

        let pelvic_joint = GenericJointBuilder::new(JointAxesMask::LIN_AXES)
            .local_anchor1(Vec3::new(0., 1.0, 0.0))
            .local_anchor2(Vec3::new(0., 0.0, 0.0))
            .coupled_axes(JointAxesMask::LIN_AXES)
            // .motor_position(
            //     JointAxis::AngY,
            //     self.pitching_arm.sign() * PI / 2.,
            //     1.,
            //     0.01,
            // )
            .motor_model(JointAxis::AngY, MotorModel::ForceBased)
            .limits(JointAxis::AngX, [-0., 0.])
            .limits(JointAxis::AngY, [-0., 0.])
            .limits(JointAxis::AngZ, [-0., 0.])
            .build();

        children
            .spawn((
                RigidBody::Dynamic,
                GravityScale(0.),
                Collider::cuboid(0.1, 0.1, 0.3),
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
                Collider::cuboid(0.1, 0.1, 0.5),
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
            .local_anchor1(Vec3::new(0., 0.0, -0.8))
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
                    0., 1.6, -2.,
                ))),
                ImpulseJoint::new(upper_torso, TypedJoint::GenericJoint(shoulder_joint)),
                BodyPartMarker::Shoulder,
            ))
            .id()
    }

    pub(crate) fn build_elbow(&self, shoulder: Entity, children: &mut ChildBuilder) -> Entity {
        let ang_x_range: [f32; 2] = match self.pitching_arm {
            PitchingArm::Left => [PI / 2. - 0.01, PI / 2. + 0.01],
            PitchingArm::Right => [-PI / 2. - 0.01, -PI / 2. + 0.01],
        };

        let elbow_joint = GenericJointBuilder::new(JointAxesMask::LIN_AXES)
            .local_anchor1(Vec3::new(0., 0.0, -0.8))
            .local_anchor2(Vec3::new(0., 0.0, 0.0))
            .coupled_axes(JointAxesMask::LIN_AXES)
            .motor_position(
                JointAxis::AngZ,
                self.pitching_arm.sign() * PI / 2.,
                1.,
                0.01,
            )
            .motor_model(JointAxis::AngZ, MotorModel::ForceBased)
            .limits(JointAxis::AngX, ang_x_range)
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
                    0., 1.6, 3.,
                ))),
                ImpulseJoint::new(shoulder, TypedJoint::GenericJoint(elbow_joint)),
                BodyPartMarker::Elbow,
            ))
            .id()
    }

    pub(crate) fn build_wrist(&self, elbow: Entity, children: &mut ChildBuilder) -> Entity {
        let wrist_joint = GenericJointBuilder::new(JointAxesMask::LIN_AXES)
            .local_anchor1(Vec3::new(0., 0.0, -0.8))
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
                    0., 1.6, -4.,
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
        body_part: &BodyPartMarker,
        entity: Entity,
        impulse_joint: &mut ImpulseJoint,
    ) {
        match body_part {
            BodyPartMarker::Pelvis => {
                let ang_y_range = match self.pitching_arm {
                    PitchingArm::Left => [-PI / 2. - 0.01, 0.01],
                    PitchingArm::Right => [-0.01, PI / 2. + 0.01],
                };

                let new_joint = GenericJointBuilder::new(JointAxesMask::LIN_AXES)
                    .local_anchor1(Vec3::new(0., 1.0, 0.0))
                    .local_anchor2(Vec3::new(0., 0.0, 0.0))
                    .coupled_axes(JointAxesMask::LIN_AXES)
                    .motor_position(
                        JointAxis::AngY,
                        self.pitching_arm.sign() * -PI / 2.,
                        1.,
                        0.01,
                    )
                    .motor_model(JointAxis::AngY, MotorModel::ForceBased)
                    .limits(JointAxis::AngX, [-0., 0.])
                    .limits(JointAxis::AngY, ang_y_range)
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
                let ang_z_target = self.pitching_arm.sign() * (PI / 2. - self.lateral_trunk_tilt);

                let new_joint = GenericJointBuilder::new(JointAxesMask::LIN_AXES)
                    .local_anchor1(Vec3::new(0., 0.6, 0.0))
                    .local_anchor2(Vec3::new(0., 0.0, 0.0))
                    .coupled_axes(JointAxesMask::LIN_AXES)
                    .motor_position(JointAxis::AngX, ang_z_target, 1., 0.01)
                    .motor_model(JointAxis::AngX, MotorModel::ForceBased)
                    .limits(JointAxis::AngY, [-0.1, 0.1])
                    .limits(JointAxis::AngX, [-0., ang_z_target + 0.1])
                    .limits(JointAxis::AngZ, [-0., 0.])
                    .build();

                impulse_joint.data = TypedJoint::GenericJoint(new_joint);
            }
            BodyPartMarker::Shoulder => {
                let new_joint = GenericJointBuilder::new(JointAxesMask::LIN_AXES)
                    .local_anchor1(Vec3::new(0., 0.0, -0.8))
                    .local_anchor2(Vec3::new(0., 0.0, 0.0))
                    .coupled_axes(JointAxesMask::LIN_AXES)
                    .motor_position(
                        JointAxis::AngZ,
                        self.pitching_arm.sign() * (PI / 2.),
                        1.,
                        0.3,
                    )
                    .motor_model(JointAxis::AngZ, MotorModel::ForceBased)
                    .limits(JointAxis::AngX, [-0., 0.])
                    .limits(JointAxis::AngY, [-0., 0.])
                    .limits(JointAxis::AngZ, [-PI + 0.01, PI - 0.01])
                    .build();
                impulse_joint.data = TypedJoint::GenericJoint(new_joint);

                // maybe not external impulse, but external force???
                commands.entity(entity).insert(ExternalImpulse::at_point(
                    0.1 * Vec3::X,
                    Vec3::new(-3., 1.6, 0.),
                    Vec3::new(0., 1.6, 0.),
                ));
            }
            _ => {}
        }
    }

    pub(crate) fn on_pelvis_break(
        &self,
        commands: &mut Commands,
        body_part: &BodyPartMarker,
        entity: Entity,
        impulse_joint: &mut ImpulseJoint,
    ) {
        match body_part {
            BodyPartMarker::Pelvis => {}
            BodyPartMarker::Elbow => {
                let new_joint = GenericJointBuilder::new(JointAxesMask::LIN_AXES)
                    .local_anchor1(Vec3::new(0., 0.0, -0.8))
                    .local_anchor2(Vec3::new(0., 0.0, 0.0))
                    .coupled_axes(JointAxesMask::LIN_AXES)
                    .motor_position(JointAxis::AngX, 0., 0.9, 0.1)
                    .motor_position(JointAxis::AngY, 0., 0.9, 0.1)
                    .motor_model(JointAxis::AngX, MotorModel::ForceBased)
                    .motor_model(JointAxis::AngY, MotorModel::ForceBased)
                    .limits(JointAxis::AngX, [-0.01, PI / 2. + 0.01])
                    .limits(JointAxis::AngY, [-0.01, PI / 2. + 0.01])
                    .limits(JointAxis::AngZ, [-0., 0.])
                    .build();

                impulse_joint.data = TypedJoint::GenericJoint(new_joint);
            }
            BodyPartMarker::Shoulder => {
                let new_joint = GenericJointBuilder::new(JointAxesMask::LIN_AXES)
                    .local_anchor1(Vec3::new(0., 0.0, -0.8))
                    .local_anchor2(Vec3::new(0., 0.0, 0.0))
                    .coupled_axes(JointAxesMask::LIN_AXES)
                    .motor_position(JointAxis::AngZ, 0., 1., 0.1)
                    .motor_model(JointAxis::AngZ, MotorModel::ForceBased)
                    .limits(JointAxis::AngX, [-0., 0.])
                    .limits(JointAxis::AngY, [-0., 0.])
                    .limits(JointAxis::AngZ, [-PI + 0.01, PI - 0.01])
                    .build();
                impulse_joint.data = TypedJoint::GenericJoint(new_joint);
            }
            _ => {}
        }
    }

    pub(crate) fn on_max_ir(
        &self,
        commands: &mut Commands,
        body_part: &BodyPartMarker,
        entity: Entity,
        impulse_joint: &mut ImpulseJoint,
    ) {
        match body_part {
            BodyPartMarker::Pelvis => {}
            BodyPartMarker::Torso => {
                let ang_z_target = self.pitching_arm.sign() * (PI / 2. - self.lateral_trunk_tilt);

                let new_joint = GenericJointBuilder::new(JointAxesMask::LIN_AXES)
                    .local_anchor1(Vec3::new(0., 0.6, 0.0))
                    .local_anchor2(Vec3::new(0., 0.0, 0.0))
                    .coupled_axes(JointAxesMask::LIN_AXES)
                    // .motor_position(JointAxis::AngY, -PI / 4., 1., 0.01)
                    // .motor_model(JointAxis::AngY, MotorModel::ForceBased)
                    .limits(JointAxis::AngX, [-0., ang_z_target + 0.1])
                    .limits(JointAxis::AngY, [-PI / 4., 0.1])
                    .limits(JointAxis::AngZ, [-0., 0.])
                    .build();

                impulse_joint.data = TypedJoint::GenericJoint(new_joint);
            }
            BodyPartMarker::Shoulder => {
                let new_joint = GenericJointBuilder::new(JointAxesMask::LIN_AXES)
                    .local_anchor1(Vec3::new(0., 0.0, -0.8))
                    .local_anchor2(Vec3::new(0., 0.0, 0.0))
                    .coupled_axes(JointAxesMask::LIN_AXES)
                    // .motor_position(JointAxis::AngZ, 0., 1., 0.1)
                    // .motor_model(JointAxis::AngZ, MotorModel::ForceBased)
                    .limits(JointAxis::AngX, [-0., 0.])
                    // .limits(JointAxis::AngY, [-0., 0.])
                    .limits(JointAxis::AngZ, [-PI + 0.01, PI - 0.01])
                    .build();
                impulse_joint.data = TypedJoint::GenericJoint(new_joint);
            }
            BodyPartMarker::Elbow => {}
            _ => {}
        }
    }
}
