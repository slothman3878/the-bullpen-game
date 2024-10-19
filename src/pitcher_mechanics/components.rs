use core::f32;

use bevy::utils::HashMap;

use crate::prelude::*;

pub(crate) const DISTANCE_CORE_HIP: f32 = 0.2;
pub(crate) const DISTANCE_CHEST_SHOULDER: f32 = 0.4;

#[derive(Debug, Reflect, Clone, Component)]
#[reflect(Component)]
pub(crate) struct BalanceWeightMarker;

#[derive(Debug, Reflect, Clone, Component)]
#[reflect(Component)]
pub(crate) struct PelvicBreakTriggerMarker;

#[derive(Debug, Reflect, Clone, Component, Hash, PartialEq, Eq, PartialOrd)]
#[reflect(Component)]
pub(crate) enum PitcherBodyPartMarker {
    BackFoot,
    BackHip,
    Core,
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
    pub(crate) fn sign(&self) -> f32 {
        match self {
            PitchingArm::Left => 1.,
            PitchingArm::Right => -1.,
        }
    }
}

#[derive(Debug, Component, Reflect, Clone, Copy, Default, PartialEq, Eq, PartialOrd)]
#[reflect(Component)]
pub(crate) enum PitchStage {
    #[default]
    WindUp,
    Stride,
    ArmCocking,
    ArmAcceleration,
    ArmDeceleration,
    // FollowThrough
}

#[derive(Debug, Component, Clone)]
pub(crate) struct PitcherParams {
    pub pitching_arm: PitchingArm,
    pub lateral_trunk_tilt: f32, // PI / 2. - lateral_trunk_tilt
    pub direction: Vec3,
    pub starting_pos: Vec3,
    //
    pub leg_length: f32,
    pub chest_length: f32,
    pub waist_length: f32,
    pub upper_arm_length: f32,
    pub forearm_length: f32,
    //
    pub torso_drop: f32,
    //
    pub body_parts: HashMap<PitcherBodyPartMarker, Entity>,
    pub ball: Option<Entity>,
    // various triggers
    pub pelvic_break: Option<Entity>,
    pub balance_weight: Option<Entity>,
}

impl Default for PitcherParams {
    fn default() -> Self {
        let body_parts = HashMap::<PitcherBodyPartMarker, Entity>::new();
        Self {
            pitching_arm: PitchingArm::default(),
            lateral_trunk_tilt: PI / 2.,
            direction: Vec3::Z,
            starting_pos: Vec3::ZERO,
            //
            leg_length: 1.0,
            chest_length: 0.375,
            waist_length: 0.25,
            upper_arm_length: 0.3,
            forearm_length: 0.3,
            //
            torso_drop: 0.3,
            //
            ball: None,
            pelvic_break: None,
            balance_weight: None,
            //
            body_parts,
        }
    }
}

impl PitcherParams {
    pub(crate) fn build_balance_weight(
        &self,
        commands: &mut Commands,
        transform: Transform,
    ) -> Entity {
        commands
            .spawn((
                RigidBody::Dynamic,
                GravityScale(1.),
                Friction::coefficient(0.),
                ColliderMassProperties::Density(10000.),
                Collider::cuboid(0.5, 0.05, 0.5),
                TransformBundle::from_transform(transform),
                BalanceWeightMarker,
            ))
            .id()
    }

    pub(crate) fn build_back_hip(
        &self,
        core: Entity,
        commands: &mut Commands,
        transform: Transform,
    ) -> Entity {
        let back_hip_joint = GenericJointBuilder::new(JointAxesMask::LIN_AXES)
            .local_anchor1(Vec3::new(0., -DISTANCE_CORE_HIP, 0.0))
            .local_anchor2(Vec3::new(0., 0.0, 0.0))
            .coupled_axes(JointAxesMask::LIN_AXES)
            .limits(JointAxis::AngX, [-0., 0.])
            .limits(JointAxis::AngY, [-0., 0.])
            .limits(JointAxis::AngZ, [-0., 0.])
            .build();

        commands
            .spawn((
                RigidBody::Dynamic,
                GravityScale(1.),
                ColliderMassProperties::Density(1000.0),
                Collider::cuboid(0.05, 0.05, 0.05),
                TransformBundle::from_transform(transform),
                ImpulseJoint::new(core, TypedJoint::GenericJoint(back_hip_joint)),
                PitcherBodyPartMarker::BackHip,
            ))
            .id()
    }

    pub(crate) fn build_back_ankle(
        &self,
        back_hip: Entity,
        commands: &mut Commands,
        transform: Transform,
    ) -> Entity {
        let back_ankle_joint = GenericJointBuilder::new(JointAxesMask::LIN_AXES)
            .local_anchor1(Vec3::new(0., -self.leg_length, 0.0))
            .local_anchor2(Vec3::new(0., 0.0, 0.0))
            .coupled_axes(JointAxesMask::LIN_AXES)
            .limits(JointAxis::AngX, [-0., 0.])
            .limits(JointAxis::AngY, [-0., 0.])
            .limits(JointAxis::AngZ, [-0., 0.])
            .build();

        // honestly more of a back foot anchor
        commands
            .spawn((
                RigidBody::KinematicVelocityBased,
                GravityScale(1.),
                ColliderMassProperties::Density(10000.0),
                Collider::cuboid(0.05, 0.05, 0.05),
                TransformBundle::from_transform(transform),
                ImpulseJoint::new(back_hip, TypedJoint::GenericJoint(back_ankle_joint)),
                PitcherBodyPartMarker::BackFoot,
            ))
            .id()
    }

    pub(crate) fn build_core(
        &self,
        balance_weight: Entity,
        commands: &mut Commands,
        transform: Transform,
    ) -> Entity {
        let balance_weight_joint = PrismaticJointBuilder::new(Vec3::Y)
            // .limits([self.leg_length - self.torso_drop, 10.])
            .build();

        commands
            .spawn((
                RigidBody::Dynamic,
                GravityScale(1.),
                ColliderMassProperties::Density(100000.0),
                Collider::cuboid(0.05, 0.05, 0.05),
                TransformBundle::from_transform(transform),
                ImpulseJoint::new(
                    balance_weight,
                    TypedJoint::PrismaticJoint(balance_weight_joint),
                ),
                PitcherBodyPartMarker::Core,
            ))
            .id()
    }

    pub(crate) fn build_pelvis(
        &self,
        core: Entity,
        commands: &mut Commands,
        transform: Transform,
    ) -> Entity {
        let pelvic_joint = GenericJointBuilder::new(JointAxesMask::LIN_AXES)
            .local_anchor1(Vec3::new(0., self.waist_length, 0.0))
            .local_anchor2(Vec3::new(0., 0.0, 0.0))
            .coupled_axes(JointAxesMask::LIN_AXES)
            .motor_model(JointAxis::AngX, MotorModel::ForceBased)
            .limits(JointAxis::AngX, [-0., 0.])
            .limits(
                JointAxis::AngY,
                [
                    self.pitching_arm.sign() * PI / 2.,
                    self.pitching_arm.sign() * PI / 2.,
                ],
            )
            .limits(JointAxis::AngZ, [-0., 0.])
            .build();

        commands
            .spawn((
                RigidBody::Dynamic,
                GravityScale(1.),
                ColliderMassProperties::Density(1000.0),
                Collider::cuboid(0.15, 0.05, 0.05),
                TransformBundle::from_transform(transform),
                ImpulseJoint::new(core, TypedJoint::GenericJoint(pelvic_joint)),
                PitcherBodyPartMarker::Pelvis,
            ))
            .id()
    }

    pub(crate) fn build_upper_torso(
        &mut self,
        pelvis: Entity,
        commands: &mut Commands,
        transform: Transform,
    ) -> Entity {
        let spinal_joint = GenericJointBuilder::new(JointAxesMask::LIN_AXES)
            .local_anchor1(Vec3::new(0., self.chest_length, 0.0))
            .local_anchor2(Vec3::new(0., 0.0, 0.0))
            .coupled_axes(JointAxesMask::LIN_AXES)
            .limits(JointAxis::AngX, [-0., 0.])
            .limits(JointAxis::AngY, [-0., 0.])
            .limits(JointAxis::AngZ, [-0., 0.])
            .build();

        commands
            .spawn((
                RigidBody::Dynamic,
                GravityScale(1.),
                Collider::cuboid(0.3, 0.05, 0.05),
                ColliderMassProperties::Density(1000.0),
                TransformBundle::from_transform(transform),
                ImpulseJoint::new(pelvis, TypedJoint::GenericJoint(spinal_joint)),
                PitcherBodyPartMarker::Torso,
            ))
            .id()
    }

    pub(crate) fn build_shoulder(
        &self,
        upper_torso: Entity,
        commands: &mut Commands,
        transform: Transform,
    ) -> Entity {
        let shoulder_joint = GenericJointBuilder::new(JointAxesMask::LIN_AXES)
            .local_anchor1(Vec3::new(
                self.pitching_arm.sign() * DISTANCE_CHEST_SHOULDER,
                0.0,
                0.,
            ))
            .local_anchor2(Vec3::new(0., 0.0, 0.0))
            .coupled_axes(JointAxesMask::LIN_AXES)
            .limits(JointAxis::AngX, [0., 0.])
            .limits(JointAxis::AngY, [0., 0.])
            .limits(JointAxis::AngZ, [0., 0.])
            .build();

        commands
            .spawn((
                RigidBody::Dynamic,
                GravityScale(1.),
                Collider::cuboid(0.05, 0.05, 0.05),
                ColliderMassProperties::Density(1000.0),
                TransformBundle::from_transform(transform),
                ImpulseJoint::new(upper_torso, TypedJoint::GenericJoint(shoulder_joint)),
                PitcherBodyPartMarker::Shoulder,
                ExternalForce::default(),
            ))
            .id()
    }

    pub(crate) fn build_elbow(
        &self,
        shoulder: Entity,
        commands: &mut Commands,
        transform: Transform,
    ) -> Entity {
        let elbow_joint = GenericJointBuilder::new(JointAxesMask::LIN_AXES)
            .local_anchor1(Vec3::new(
                self.pitching_arm.sign() * self.upper_arm_length,
                0.0,
                0.,
            ))
            .local_anchor2(Vec3::new(0., 0.0, 0.0))
            .coupled_axes(JointAxesMask::LIN_AXES)
            .limits(JointAxis::AngX, [0., 0.])
            .limits(JointAxis::AngY, [0., 0.])
            .limits(JointAxis::AngZ, [0., 0.])
            .build();

        commands
            .spawn((
                RigidBody::Dynamic,
                GravityScale(1.),
                Collider::cuboid(0.05, 0.05, 0.05),
                ColliderMassProperties::Density(1000.0),
                TransformBundle::from_transform(transform),
                ImpulseJoint::new(shoulder, TypedJoint::GenericJoint(elbow_joint)),
                PitcherBodyPartMarker::Elbow,
            ))
            .id()
    }

    pub(crate) fn build_wrist(
        &self,
        elbow: Entity,
        commands: &mut Commands,
        transform: Transform,
    ) -> Entity {
        let wrist_joint = GenericJointBuilder::new(JointAxesMask::LIN_AXES)
            .local_anchor1(Vec3::new(
                self.pitching_arm.sign() * self.forearm_length,
                0.0,
                0.,
            ))
            .local_anchor2(Vec3::new(0., 0.0, 0.0))
            .coupled_axes(JointAxesMask::LIN_AXES)
            .limits(JointAxis::AngX, [0., 0.])
            .limits(JointAxis::AngY, [0., 0.])
            .limits(JointAxis::AngZ, [0., 0.])
            .build();

        commands
            .spawn((
                RigidBody::Dynamic,
                GravityScale(1.),
                Collider::cuboid(0.05, 0.05, 0.05),
                ColliderMassProperties::Density(1000.0),
                TransformBundle::from_transform(transform),
                ImpulseJoint::new(elbow, TypedJoint::GenericJoint(wrist_joint)),
                Velocity::default(),
                PitcherBodyPartMarker::Wrist,
            ))
            .id()
    }

    pub(crate) fn build_ball(
        &self,
        wrist: Entity,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        transform: Transform,
    ) -> Entity {
        let grab = FixedJointBuilder::new()
            .local_anchor1(Vec3::new(0., 0.0, 0.1))
            .local_anchor2(Vec3::new(0., 0.0, 0.0));

        commands
            .spawn((
                Name::new("ball"),
                TransformBundle::from_transform(transform),
                ImpulseJoint::new(wrist, grab),
                Velocity::default(),
                ExternalForce::default(),
                Restitution {
                    coefficient: 0.546,
                    combine_rule: CoefficientCombineRule::Min,
                },
                PitcherBodyPartMarker::Wrist,
                BaseballFlightBundle::default(),
            ))
            .with_children(|children| {
                children.spawn((PbrBundle {
                    mesh: meshes.add(Sphere::new(0.037)).into(), // default 0.075
                    material: materials.add(Color::srgb(0.9, 0.9, 0.9)),
                    ..default()
                },));
            })
            .id()
    }

    pub(crate) fn on_knee_up(
        &self,
        commands: &mut Commands,
        body_part: &PitcherBodyPartMarker,
        entity: Entity,
        impulse_joint: &mut ImpulseJoint,
    ) {
        match body_part {
            PitcherBodyPartMarker::Core => {}
            PitcherBodyPartMarker::BackFoot => {
                let new_joint = GenericJointBuilder::new(JointAxesMask::LIN_AXES)
                    .local_anchor1(Vec3::new(0., -self.leg_length, 0.0))
                    .local_anchor2(Vec3::new(0., 0.0, 0.0))
                    .coupled_axes(JointAxesMask::LIN_AXES)
                    // .limits(JointAxis::AngX, [-0., 0.])
                    .limits(JointAxis::AngY, [-0., 0.])
                    .limits(JointAxis::AngZ, [-0., 0.])
                    .build();

                impulse_joint.data = TypedJoint::GenericJoint(new_joint);
            }
            PitcherBodyPartMarker::BackHip => {
                let new_joint = GenericJointBuilder::new(JointAxesMask::LIN_AXES)
                    .local_anchor1(Vec3::new(0., -0.2, 0.0))
                    .local_anchor2(Vec3::new(0., 0.0, 0.0))
                    .coupled_axes(JointAxesMask::LIN_AXES)
                    // .limits(JointAxis::AngX, [-0., 0.])
                    .limits(JointAxis::AngY, [-0., 0.])
                    .limits(JointAxis::AngZ, [-0., 0.])
                    .build();

                impulse_joint.data = TypedJoint::GenericJoint(new_joint);
            }
            _ => {}
        }
    }

    pub(crate) fn on_foot_contact(
        &self,
        commands: &mut Commands,
        body_part: &PitcherBodyPartMarker,
        entity: Entity,
        impulse_joint: &mut ImpulseJoint,
    ) {
        match body_part {
            PitcherBodyPartMarker::Core => {}
            PitcherBodyPartMarker::Pelvis => {
                let ang_y_range = match self.pitching_arm {
                    PitchingArm::Left => [-0.1, PI / 2.],
                    PitchingArm::Right => [-PI / 2., 0.1],
                };

                let ang_z_target = self.pitching_arm.sign() * (PI / 2. - self.lateral_trunk_tilt);
                let ang_z_range = match self.pitching_arm {
                    PitchingArm::Left => [-0., ang_z_target],
                    PitchingArm::Right => [ang_z_target, 0.],
                };

                let new_joint = GenericJointBuilder::new(JointAxesMask::LIN_AXES)
                    .local_anchor1(Vec3::new(0., self.waist_length, 0.0))
                    .local_anchor2(Vec3::new(0., 0.0, 0.0))
                    .coupled_axes(JointAxesMask::LIN_AXES)
                    .motor_position(
                        JointAxis::AngY,
                        self.pitching_arm.sign() * 0.1, // arm dependent
                        1000.,
                        1.,
                    )
                    .motor_model(JointAxis::AngZ, MotorModel::ForceBased)
                    .motor_model(JointAxis::AngY, MotorModel::ForceBased)
                    .limits(JointAxis::AngX, [-0., 0.])
                    .limits(JointAxis::AngY, ang_y_range)
                    .limits(JointAxis::AngZ, ang_z_range)
                    // .limits(JointAxis::AngZ, [-0., 0.])
                    .build();

                impulse_joint.data = TypedJoint::GenericJoint(new_joint);

                // mostly to kickstart the motor
                commands.entity(entity).insert(ExternalImpulse::at_point(
                    self.pitching_arm.sign() * 0.01 * Vec3::X,
                    Vec3::new(0., 1., -0.2),
                    Vec3::new(0., 1., 0.),
                ));
            }
            PitcherBodyPartMarker::Torso => {
                let ang_z_target = self.pitching_arm.sign() * (PI / 2. - self.lateral_trunk_tilt);

                let ang_y_range = match self.pitching_arm {
                    PitchingArm::Left => [-PI / 6., 1.],
                    PitchingArm::Right => [-1., PI / 6.],
                };
                let ang_z_range = match self.pitching_arm {
                    PitchingArm::Left => [-0., ang_z_target],
                    PitchingArm::Right => [ang_z_target, 0.],
                };

                let new_joint = GenericJointBuilder::new(JointAxesMask::LIN_AXES)
                    .local_anchor1(Vec3::new(0., self.chest_length, 0.0))
                    .local_anchor2(Vec3::new(0., 0.0, 0.0))
                    .coupled_axes(JointAxesMask::LIN_AXES)
                    .motor_position(
                        JointAxis::AngY,
                        0., // self.pitching_arm.sign() * PI / 2.,
                        1000.,
                        0.1,
                    )
                    .motor_model(JointAxis::AngZ, MotorModel::ForceBased)
                    .limits(JointAxis::AngX, [-0.1, 0.1])
                    .limits(JointAxis::AngY, ang_y_range)
                    // .limits(JointAxis::AngZ, ang_z_range)
                    .limits(JointAxis::AngZ, [-0., 0.])
                    .build();

                impulse_joint.data = TypedJoint::GenericJoint(new_joint);
            }
            PitcherBodyPartMarker::Shoulder => {}
            _ => {}
        }
    }

    // a.k.a. pelvis break
    pub(crate) fn on_max_er(
        &self,
        commands: &mut Commands,
        body_part: &PitcherBodyPartMarker,
        entity: Entity,
        impulse_joint: &mut ImpulseJoint,
        global_translation: Vec3,
    ) {
        match body_part {
            PitcherBodyPartMarker::Pelvis => {
                let ang_y_range = match self.pitching_arm {
                    PitchingArm::Left => [-0.1, 1.],
                    PitchingArm::Right => [-1., 0.1],
                };

                let ang_z_target = self.pitching_arm.sign() * (PI / 2. - self.lateral_trunk_tilt);
                let ang_z_range = match self.pitching_arm {
                    PitchingArm::Left => [-0., ang_z_target],
                    PitchingArm::Right => [ang_z_target, 0.],
                };

                let new_joint = GenericJointBuilder::new(JointAxesMask::LIN_AXES)
                    .local_anchor1(Vec3::new(0., self.waist_length, 0.0))
                    .local_anchor2(Vec3::new(0., 0.0, 0.0))
                    .coupled_axes(JointAxesMask::LIN_AXES)
                    .motor_position(
                        JointAxis::AngY,
                        self.pitching_arm.sign() * 0.1, // arm dependent
                        800.,
                        1.,
                    )
                    .motor_position(
                        JointAxis::AngZ,
                        ang_z_target, // self.pitching_arm.sign() * PI / 2.,
                        800.,
                        1.,
                    )
                    .motor_model(JointAxis::AngZ, MotorModel::ForceBased)
                    .limits(JointAxis::AngX, [-0., PI / 4.])
                    .limits(JointAxis::AngY, ang_y_range)
                    .limits(JointAxis::AngZ, ang_z_range)
                    .build();

                impulse_joint.data = TypedJoint::GenericJoint(new_joint);
            }
            PitcherBodyPartMarker::Torso => {}
            PitcherBodyPartMarker::Elbow => {}
            PitcherBodyPartMarker::Shoulder => {}
            _ => {}
        }
    }

    pub(crate) fn on_max_ir(
        &self,
        commands: &mut Commands,
        body_part: &PitcherBodyPartMarker,
        entity: Entity,
        impulse_joint: &mut ImpulseJoint,
    ) {
        match body_part {
            PitcherBodyPartMarker::Pelvis => {
                let ang_y_range = match self.pitching_arm {
                    PitchingArm::Left => [-0.1, 1.],
                    PitchingArm::Right => [-1., 0.1],
                };

                let ang_z_target = self.pitching_arm.sign() * (PI / 2. - self.lateral_trunk_tilt);
                let ang_z_range = match self.pitching_arm {
                    PitchingArm::Left => [ang_z_target - 0.2, ang_z_target],
                    PitchingArm::Right => [ang_z_target, ang_z_target + 0.2],
                };

                let new_joint = GenericJointBuilder::new(JointAxesMask::LIN_AXES)
                    .local_anchor1(Vec3::new(0., self.waist_length, 0.0))
                    .local_anchor2(Vec3::new(0., 0.0, 0.0))
                    .coupled_axes(JointAxesMask::LIN_AXES)
                    .motor_position(
                        JointAxis::AngY,
                        self.pitching_arm.sign() * 0.1, // arm dependent
                        800.,
                        1.,
                    )
                    .limits(JointAxis::AngX, [-0., PI / 4.])
                    .limits(JointAxis::AngY, ang_y_range)
                    .limits(JointAxis::AngZ, ang_z_range)
                    .build();

                impulse_joint.data = TypedJoint::GenericJoint(new_joint);
            }
            PitcherBodyPartMarker::Torso => {
                let ang_z_target = self.pitching_arm.sign() * (PI / 2. - self.lateral_trunk_tilt);

                let ang_y_range = match self.pitching_arm {
                    PitchingArm::Left => [-PI / 6., 0.1],
                    PitchingArm::Right => [-0.1, PI / 6.],
                };
                let ang_z_range = match self.pitching_arm {
                    PitchingArm::Left => [ang_z_target - 0.1, ang_z_target],
                    PitchingArm::Right => [ang_z_target, ang_z_target + 0.1],
                };

                let new_joint = GenericJointBuilder::new(JointAxesMask::LIN_AXES)
                    .local_anchor1(Vec3::new(0., self.chest_length, 0.0))
                    .local_anchor2(Vec3::new(0., 0.0, 0.0))
                    .coupled_axes(JointAxesMask::LIN_AXES)
                    .limits(JointAxis::AngX, [-0.1, 0.1])
                    .limits(JointAxis::AngY, ang_y_range)
                    .limits(JointAxis::AngZ, [-0., 0.])
                    .build();

                impulse_joint.data = TypedJoint::GenericJoint(new_joint);
            }
            PitcherBodyPartMarker::Shoulder => {
                let new_joint = GenericJointBuilder::new(JointAxesMask::LIN_AXES)
                    .local_anchor1(Vec3::new(self.pitching_arm.sign() * 0.4, 0.0, 0.))
                    .local_anchor2(Vec3::new(0., 0.0, 0.0))
                    .coupled_axes(JointAxesMask::LIN_AXES)
                    .limits(JointAxis::AngX, [-0., 0.])
                    .limits(JointAxis::AngZ, [0., 0.])
                    .build();
                impulse_joint.data = TypedJoint::GenericJoint(new_joint);
            }
            PitcherBodyPartMarker::Elbow => {}
            _ => {}
        }
    }
}
