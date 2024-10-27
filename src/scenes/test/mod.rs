use core::f32;

use bevy::pbr::CascadeShadowConfigBuilder;

use crate::prelude::*;

pub(crate) mod prelude {
    pub(crate) use super::*;
}

const PI: f32 = std::f32::consts::PI;

// bullpen scene
#[derive(Debug, Reflect, States, Hash, Eq, PartialEq, Clone, Copy)]
pub(crate) struct TestScene;

impl GameScene for TestScene {
    fn configure_set(&self, app: &mut App) {
        app.configure_sets(
            OnEnter(*self),
            ((GameScenesSet::OnEnterSet(*self),).run_if(in_state(*self)),),
        )
        .configure_sets(
            Update,
            GameScenesSet::UpdateSet(*self).run_if(in_state(*self)),
        )
        .configure_sets(
            OnExit(*self),
            GameScenesSet::OnExitSet(*self).run_if(in_state(*self)),
        );
    }

    fn register_type(&self, app: &mut App) {
        app.register_type::<GameSceneMarker<Self>>();
    }
}

impl Plugin for TestScene {
    fn build(&self, app: &mut App) {
        self.register_type(app);
        self.configure_set(app);

        app.add_event::<PitchStageTransitionEvents>();

        app.add_systems(
            OnEnter(Self),
            (
                spawn_camera,
                setup_sun,
                // spawn_arms,
                // spawn_camera.after(setup_scene),
            )
                .chain()
                .in_set(GameScenesSet::OnEnterSet(*self)),
        )
        .add_systems(
            Update,
            (
                max_er.run_if(input_just_released(KeyCode::KeyR)),
                release.run_if(input_just_released(MouseButton::Left)),
                // push_shoulder.run_if(input_pressed(MouseButton::Left)),
            )
                .chain()
                .in_set(GameScenesSet::UpdateSet(*self)),
        );
    }
}

fn spawn_camera(mut commands: Commands) {
    info!("spawn camera");
    commands.spawn((
        Name::new("fly cam"),
        FlyCam,
        Camera3dBundle {
            camera: Camera {
                is_active: true,
                order: 0,
                ..default()
            },
            transform: Transform::from_xyz(0., 1.6, -5.)
                .looking_at(Vec3::new(0., 1.6, 0.), Vec3::Y),
            ..default()
        },
    ));
}

pub(crate) fn setup_sun(mut commands: Commands) {
    // directional 'sun' light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(PI / 4.),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .into(),
        ..default()
    });
}

#[derive(Component)]
enum BodyPartMarker {
    Pelvis,
    Torso,
    Shoulder,
    Elbow,
}

// this is at foot contact
fn max_er(
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
fn release(mut query_arm_part: Query<(Entity, &mut ImpulseJoint, &BodyPartMarker)>) {
    for (_, mut impulse_joint, arm_part) in query_arm_part.iter_mut() {
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
            _ => {}
        }
    }
}

#[derive(Debug, Event)]
pub(crate) enum PitchStageTransitionEvents {
    FootContact(Entity),
    PelvisBreak(Entity),
    Release(Entity),
}

fn on_pitch_stage_transition_event(
    mut ev_pitch_stage_transition_event: EventReader<PitchStageTransitionEvents>,
    mut commands: Commands,
    mut query_pitcher: Query<Entity, With<PitcherMarker>>,
    mut query_body_part: Query<(Entity, &mut ImpulseJoint, &BodyPartMarker)>,
) {
    for ev in ev_pitch_stage_transition_event.read() {
        match ev {
            PitchStageTransitionEvents::FootContact(entity) => {
                // need to get children of entity
                if let Ok((entity, mut impulse_joint, arm_part)) = query_body_part.get(*entity) {}
            }
            PitchStageTransitionEvents::PelvisBreak(entity) => {
                // commands.schedule_on_update(break_system);
            }
            PitchStageTransitionEvents::Release(entity) => {
                // commands.schedule_on_update(release_system);
            }
        }
    }
}

// #[derive(Debug, Clone, Copy)]
// pub(crate) enum PitchingArm {
//     Left,
//     Right,
// }

// impl PitchingArm {
//     fn sign(&self) -> f32 {
//         match self {
//             PitchingArm::Left => 1.,
//             PitchingArm::Right => -1.,
//         }
//     }
// }

// #[derive(Debug, Clone, Copy, Default)]
// pub(crate) enum PitchStage {
//     #[default]
//     // KneeUp
//     Stride,
//     ArmCocking,
//     ArmAcceleration,
//     ArmDeceleration,
//     // FollowThrough
// }

// #[derive(Debug, Component, Clone, Copy)]
// pub(crate) struct PitcherParams {
//     pub height: f32,
//     pub pitching_arm: PitchingArm,
//     pub lateral_trunk_tilt: f32,
// }

// impl PitcherParams {
//     fn build_core(&self, children: &mut ChildBuilder) -> Entity {
//         children
//             .spawn((
//                 RigidBody::KinematicPositionBased,
//                 GravityScale(0.),
//                 ColliderMassProperties::Density(100.),
//                 Collider::cuboid(0.1, 0.1, 0.1),
//                 TransformBundle::from_transform(Transform::from_translation(Vec3::new(0., 0., 0.))),
//             ))
//             .id()
//     }

//     fn build_pelvis(&self, core: Entity, children: &mut ChildBuilder) -> Entity {
//         let ang_y_range: [f32; 2] = match self.pitching_arm {
//             PitchingArm::Left => [-0.01, PI / 2. + 0.01],
//             PitchingArm::Right => [-PI / 2. - 0.01, 0.01],
//         };

//         let pelvic_joint = GenericJointBuilder::new(JointAxesMask::LIN_AXES)
//             .local_anchor1(Vec3::new(0., 1.0, 0.0))
//             .local_anchor2(Vec3::new(0., 0.0, 0.0))
//             .coupled_axes(JointAxesMask::LIN_AXES)
//             .motor_position(
//                 JointAxis::AngY,
//                 self.pitching_arm.sign() * PI / 2.,
//                 1.,
//                 0.01,
//             )
//             .motor_model(JointAxis::AngY, MotorModel::ForceBased)
//             .limits(JointAxis::AngX, [-0., 0.])
//             .limits(JointAxis::AngY, ang_y_range)
//             .limits(JointAxis::AngZ, [-0., 0.])
//             .build();

//         children
//             .spawn((
//                 RigidBody::Dynamic,
//                 GravityScale(0.),
//                 Collider::cuboid(0.3, 0.1, 0.1),
//                 TransformBundle::from_transform(Transform::from_translation(Vec3::new(0., 1., 0.))),
//                 ImpulseJoint::new(core, TypedJoint::GenericJoint(pelvic_joint)),
//                 BodyPartMarker::Pelvis,
//             ))
//             .id()
//     }

//     fn build_upper_torso(&self, pelvis: Entity, children: &mut ChildBuilder) -> Entity {
//         let spinal_joint = GenericJointBuilder::new(JointAxesMask::LIN_AXES)
//             .local_anchor1(Vec3::new(0., 0.6, 0.0))
//             .local_anchor2(Vec3::new(0., 0.0, 0.0))
//             .coupled_axes(JointAxesMask::LIN_AXES)
//             // .motor_position(JointAxis::AngY, 0., 1., 0.01)
//             .motor_model(JointAxis::AngY, MotorModel::ForceBased)
//             .limits(JointAxis::AngX, [-0., 0.])
//             .limits(JointAxis::AngY, [-0.1, 0.1])
//             .limits(JointAxis::AngZ, [-0., 0.])
//             .build();

//         children
//             .spawn((
//                 RigidBody::Dynamic,
//                 GravityScale(0.),
//                 Collider::cuboid(0.5, 0.1, 0.1),
//                 ColliderMassProperties::Density(10.0),
//                 TransformBundle::from_transform(Transform::from_translation(Vec3::new(
//                     0., 1.6, 0.,
//                 ))),
//                 ImpulseJoint::new(pelvis, TypedJoint::GenericJoint(spinal_joint)),
//                 BodyPartMarker::Torso,
//             ))
//             .id()
//     }

//     fn build_shoulder(&self, upper_torso: Entity, children: &mut ChildBuilder) -> Entity {
//         let shoulder_joint = GenericJointBuilder::new(JointAxesMask::LIN_AXES)
//             .local_anchor1(Vec3::new(self.pitching_arm.sign() * 0.8, 0.0, 0.0))
//             .local_anchor2(Vec3::new(0., 0.0, 0.0))
//             .coupled_axes(JointAxesMask::LIN_AXES)
//             // .motor_position(JointAxis::AngX, 0., 0.1, 0.01)
//             // .motor_model(JointAxis::AngX, MotorModel::ForceBased)
//             .limits(JointAxis::AngX, [-0., 0.])
//             .limits(JointAxis::AngY, [-0., 0.])
//             .limits(JointAxis::AngZ, [-0., 0.])
//             .build();

//         children
//             .spawn((
//                 RigidBody::Dynamic,
//                 GravityScale(0.),
//                 Collider::cuboid(0.05, 0.05, 0.05),
//                 ColliderMassProperties::Density(10.0),
//                 TransformBundle::from_transform(Transform::from_translation(Vec3::new(
//                     self.pitching_arm.sign() * 2.,
//                     1.6,
//                     0.,
//                 ))),
//                 ImpulseJoint::new(upper_torso, TypedJoint::GenericJoint(shoulder_joint)),
//                 BodyPartMarker::Shoulder,
//             ))
//             .id()
//     }

//     fn build_elbow(&self, shoulder: Entity, children: &mut ChildBuilder) -> Entity {
//         let ang_z_range: [f32; 2] = match self.pitching_arm {
//             PitchingArm::Left => [-0.01, PI / 2. + 0.01],
//             PitchingArm::Right => [-PI / 2. - 0.01, 0.01],
//         };

//         let elbow_joint = GenericJointBuilder::new(JointAxesMask::LIN_AXES)
//             .local_anchor1(Vec3::new(self.pitching_arm.sign() * 0.8, 0.0, 0.0))
//             .local_anchor2(Vec3::new(0., 0.0, 0.0))
//             .coupled_axes(JointAxesMask::LIN_AXES)
//             .motor_position(
//                 JointAxis::AngZ,
//                 self.pitching_arm.sign() * PI / 2.,
//                 1.,
//                 0.01,
//             )
//             .motor_model(JointAxis::AngZ, MotorModel::ForceBased)
//             .limits(JointAxis::AngX, [-0., 0.])
//             .limits(JointAxis::AngY, [-0., 0.])
//             .limits(JointAxis::AngZ, ang_z_range)
//             .build();

//         children
//             .spawn((
//                 RigidBody::Dynamic,
//                 GravityScale(0.),
//                 Collider::cuboid(0.05, 0.05, 0.05),
//                 ColliderMassProperties::Density(10.0),
//                 TransformBundle::from_transform(Transform::from_translation(Vec3::new(
//                     self.pitching_arm.sign() * 3.,
//                     1.6,
//                     0.,
//                 ))),
//                 ImpulseJoint::new(shoulder, TypedJoint::GenericJoint(elbow_joint)),
//                 BodyPartMarker::Elbow,
//             ))
//             .id()
//     }

//     fn build_wrist(&self, elbow: Entity, children: &mut ChildBuilder) -> Entity {
//         let ang_x_range: [f32; 2] = match self.pitching_arm {
//             PitchingArm::Left => [-0.01, 0.01],
//             PitchingArm::Right => [-0.01, 0.01],
//         };

//         let wrist_joint = GenericJointBuilder::new(JointAxesMask::LIN_AXES)
//             .local_anchor1(Vec3::new(self.pitching_arm.sign() * 0.8, 0.0, 0.0))
//             .local_anchor2(Vec3::new(0., 0.0, 0.0))
//             .coupled_axes(JointAxesMask::LIN_AXES)
//             .motor_position(JointAxis::AngZ, 0., 1., 0.01)
//             .motor_model(JointAxis::AngZ, MotorModel::ForceBased)
//             .limits(JointAxis::AngX, [-0.01, 0.01])
//             .limits(JointAxis::AngY, [-0.01, 0.01])
//             .limits(JointAxis::AngZ, [-0.01, 0.01])
//             .build();

//         children
//             .spawn((
//                 RigidBody::Dynamic,
//                 GravityScale(0.),
//                 Collider::cuboid(0.05, 0.05, 0.05),
//                 ColliderMassProperties::Density(10.0),
//                 TransformBundle::from_transform(Transform::from_translation(Vec3::new(
//                     self.pitching_arm.sign() * 4.,
//                     1.6,
//                     0.,
//                 ))),
//                 ImpulseJoint::new(elbow, TypedJoint::GenericJoint(wrist_joint)),
//             ))
//             .id()
//     }

//     fn on_foot_contact(
//         &self,
//         commands: &mut Commands,
//         entity: Entity,
//         impulse_joint: &mut ImpulseJoint,
//         body_part: &mut BodyPartMarker,
//     ) {
//         match body_part {
//             BodyPartMarker::Pelvis => {
//                 let new_joint = GenericJointBuilder::new(JointAxesMask::LIN_AXES)
//                     .local_anchor1(Vec3::new(0., 1.0, 0.0))
//                     .local_anchor2(Vec3::new(0., 0.0, 0.0))
//                     .coupled_axes(JointAxesMask::LIN_AXES)
//                     // .motor_position(JointAxis::AngY, 0., 1., 0.0001)
//                     // .motor_model(JointAxis::AngY, MotorModel::ForceBased)
//                     .limits(JointAxis::AngX, [-0., 0.])
//                     .limits(JointAxis::AngY, [-0.01, PI / 2. + 0.01])
//                     .limits(JointAxis::AngZ, [-0., 0.])
//                     .build();

//                 impulse_joint.data = TypedJoint::GenericJoint(new_joint);

//                 commands.entity(entity).insert(ExternalImpulse::at_point(
//                     1. * Vec3::X,
//                     Vec3::new(0., 1., -0.2),
//                     Vec3::new(0., 1., 0.),
//                 ));
//             }
//             BodyPartMarker::Torso => {
//                 let new_joint = GenericJointBuilder::new(JointAxesMask::LIN_AXES)
//                     .local_anchor1(Vec3::new(0., 0.6, 0.0))
//                     .local_anchor2(Vec3::new(0., 0.0, 0.0))
//                     .coupled_axes(JointAxesMask::LIN_AXES)
//                     .motor_position(JointAxis::AngZ, self.lateral_trunk_tilt, 1., 0.01)
//                     .motor_model(JointAxis::AngZ, MotorModel::ForceBased)
//                     .limits(JointAxis::AngX, [-0., 0.])
//                     .limits(JointAxis::AngY, [-0.1, 0.1])
//                     .limits(JointAxis::AngZ, [-0., self.lateral_trunk_tilt])
//                     .build();

//                 impulse_joint.data = TypedJoint::GenericJoint(new_joint);
//             }
//             BodyPartMarker::Shoulder => {
//                 let new_joint = GenericJointBuilder::new(JointAxesMask::LIN_AXES)
//                     .local_anchor1(Vec3::new(0.8, 0.0, 0.0))
//                     .local_anchor2(Vec3::new(0., 0.0, 0.0))
//                     .coupled_axes(JointAxesMask::LIN_AXES)
//                     .motor_position(JointAxis::AngX, -PI / 2., 0.9, 0.1)
//                     .motor_model(JointAxis::AngX, MotorModel::ForceBased)
//                     // .limits(JointAxis::AngX, [-0.01, PI / 2. + 0.01])
//                     .limits(JointAxis::AngY, [-0., 0.])
//                     .limits(JointAxis::AngZ, [-0., 0.])
//                     .build();
//                 impulse_joint.data = TypedJoint::GenericJoint(new_joint);
//             }
//             _ => {}
//         }
//     }
// }

// pub(crate) fn spawn_arms(mut commands: Commands) {
//     let params = PitcherParams {
//         height: 1.85,
//         pitching_arm: PitchingArm::Left,
//         lateral_trunk_tilt: PI / 4.,
//     };
//     commands
//         .spawn((
//             params.clone(),
//             TransformBundle::from_transform(Transform::from_translation(Vec3::ZERO)),
//         ))
//         .with_children(|children| {
//             let core = params.build_core(children);

//             let pelvis = params.build_pelvis(core, children);

//             let upper_torso = params.build_upper_torso(pelvis, children);

//             let shoulder = params.build_shoulder(upper_torso, children);

//             let elbow = params.build_elbow(shoulder, children);

//             let wrist = params.build_wrist(elbow, children);
//         });
// }
