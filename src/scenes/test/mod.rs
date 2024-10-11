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

        app.add_systems(
            OnEnter(Self),
            (
                spawn_camera,
                setup_sun,
                spawn_arms,
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
enum ArmPartMarker {
    Torso,
    Shoulder,
    Elbow,
}

#[derive(Component)]
struct Pelvis;

fn push_shoulder(
    mut commands: Commands,
    query_shoulder: Query<(Entity, &Transform, &ArmPartMarker)>,
) {
    for (entity, transform, arm_part) in query_shoulder.iter() {
        match arm_part {
            ArmPartMarker::Torso => {
                commands.entity(entity).insert(ExternalImpulse {
                    // impulse: Vec3::new(0., 0., 0.2),
                    torque_impulse: 0.03 * transform.rotation.mul_vec3(Vec3::Y),
                    ..default()
                });
            }
            ArmPartMarker::Shoulder => {
                commands.entity(entity).insert(ExternalImpulse {
                    // impulse: Vec3::new(0., 0., 0.2),
                    torque_impulse: -0.03 * transform.rotation.mul_vec3(Vec3::X),
                    ..default()
                });
            }
            ArmPartMarker::Elbow => {
                commands.entity(entity).insert(ExternalImpulse {
                    // impulse: Vec3::new(0., 0., 0.2),
                    torque_impulse: 0.03 * transform.rotation.mul_vec3(Vec3::Z),
                    ..default()
                });
            }
        }
    }
}

fn max_er(mut query_pelvis: Query<(Entity, &mut ImpulseJoint), With<Pelvis>>) {
    for (_, mut impulse_joint) in query_pelvis.iter_mut() {
        let new_joint = GenericJointBuilder::new(JointAxesMask::LIN_AXES)
            .local_anchor1(Vec3::new(0., 1.0, 0.0))
            .local_anchor2(Vec3::new(0., 0.0, 0.0))
            // .local_axis1(Vec3::Y)
            // .local_axis2(Vec3::Y)
            .coupled_axes(JointAxesMask::LIN_AXES)
            .motor_position(JointAxis::AngY, 0., 1., 0.1)
            .motor_model(JointAxis::AngY, MotorModel::ForceBased)
            .limits(JointAxis::AngX, [-0.01, 0.01])
            // .limits(JointAxis::AngY, [0. - 0.01, 0. + 0.01])
            .limits(JointAxis::AngZ, [-0.01, 0.01])
            .build();

        impulse_joint.data = TypedJoint::GenericJoint(new_joint);
    }
}

fn release(mut query_arm_part: Query<(Entity, &mut ImpulseJoint, &ArmPartMarker)>) {
    for (_, mut impulse_joint, arm_part) in query_arm_part.iter_mut() {
        match arm_part {
            ArmPartMarker::Elbow => {
                let new_joint = GenericJointBuilder::new(JointAxesMask::LIN_AXES)
                    .local_anchor1(Vec3::new(0.8, 0.0, 0.0))
                    .local_anchor2(Vec3::new(0., 0.0, 0.0))
                    // .local_axis1(Vec3::Y)
                    // .local_axis2(Vec3::Y)
                    .coupled_axes(JointAxesMask::LIN_AXES)
                    // .motor_position(JointAxis::AngZ, 0., 0.9, 0.1)
                    // .motor_model(JointAxis::AngZ, MotorModel::ForceBased)
                    .limits(JointAxis::AngX, [-0.01, 0.01])
                    // .limits(JointAxis::AngY, [-0.01, 0.01])
                    // .limits(JointAxis::AngZ, [PI / 2. - 0.01, PI / 2. + 0.01])
                    .build();

                impulse_joint.data = TypedJoint::GenericJoint(new_joint);
            }
            ArmPartMarker::Shoulder => {
                let new_joint = GenericJointBuilder::new(JointAxesMask::LIN_AXES)
                    .local_anchor1(Vec3::new(0.8, 0.0, 0.0))
                    .local_anchor2(Vec3::new(0., 0.0, 0.0))
                    // .local_axis1(Vec3::Y)
                    // .local_axis2(Vec3::Y)
                    .coupled_axes(JointAxesMask::LIN_AXES)
                    .motor_position(JointAxis::AngX, 0., 0.1, 0.01)
                    .motor_model(JointAxis::AngX, MotorModel::ForceBased)
                    // .limits(JointAxis::AngX, [-0.01, PI / 2. + 0.01])
                    .limits(JointAxis::AngY, [-0.01, 0.01])
                    .limits(JointAxis::AngZ, [-0.01, 0.01])
                    .build();
                impulse_joint.data = TypedJoint::GenericJoint(new_joint);
            }
            _ => {}
        }
    }
}

pub(crate) fn spawn_arms(mut commands: Commands) {
    let core = commands
        .spawn((
            RigidBody::KinematicPositionBased,
            GravityScale(0.),
            ColliderMassProperties::Density(100.),
            Collider::cuboid(0.1, 0.1, 0.1),
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(0., 0., 0.))),
        ))
        .id();

    let pelvic_joint = GenericJointBuilder::new(JointAxesMask::LIN_AXES)
        .local_anchor1(Vec3::new(0., 1.0, 0.0))
        .local_anchor2(Vec3::new(0., 0.0, 0.0))
        // .local_axis1(Vec3::Y)
        // .local_axis2(Vec3::Y)
        .coupled_axes(JointAxesMask::LIN_AXES)
        .motor_position(JointAxis::AngY, PI / 2., 1., 0.0001)
        .motor_model(JointAxis::AngY, MotorModel::ForceBased)
        .limits(JointAxis::AngX, [-0.01, 0.01])
        .limits(JointAxis::AngY, [-PI / 2. - 0.01, PI / 2. + 0.01])
        .limits(JointAxis::AngZ, [-0.01, 0.01])
        .build();

    let pelvis = commands
        .spawn((
            RigidBody::Dynamic,
            GravityScale(0.),
            Collider::cuboid(0.3, 0.1, 0.1),
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(0., 1., 0.))),
            ImpulseJoint::new(core, TypedJoint::GenericJoint(pelvic_joint)),
            Pelvis,
        ))
        .id();

    let spinal_joint = GenericJointBuilder::new(JointAxesMask::LIN_AXES)
        .local_anchor1(Vec3::new(0., 0.6, 0.0))
        .local_anchor2(Vec3::new(0., 0.0, 0.0))
        // .local_axis1(Vec3::Y)
        // .local_axis2(Vec3::Y)
        .coupled_axes(JointAxesMask::LIN_AXES)
        .motor_position(JointAxis::AngY, 0., 1., 0.1)
        .motor_model(JointAxis::AngY, MotorModel::ForceBased)
        .limits(JointAxis::AngX, [-0.01, 0.01])
        .limits(JointAxis::AngY, [-0.01, 0.01])
        .limits(JointAxis::AngZ, [-0.01, 0.01])
        .build();

    let upper_torso = commands
        .spawn((
            RigidBody::Dynamic,
            GravityScale(0.),
            Collider::cuboid(0.5, 0.1, 0.1),
            ColliderMassProperties::Density(10.0),
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(0., 1.6, 0.))),
            ImpulseJoint::new(pelvis, TypedJoint::GenericJoint(spinal_joint)),
            ArmPartMarker::Torso,
        ))
        .id();

    let shoulder_joint = GenericJointBuilder::new(JointAxesMask::LIN_AXES)
        .local_anchor1(Vec3::new(0.8, 0.0, 0.0))
        .local_anchor2(Vec3::new(0., 0.0, 0.0))
        // .local_axis1(Vec3::Y)
        // .local_axis2(Vec3::Y)
        .coupled_axes(JointAxesMask::LIN_AXES)
        .motor_position(JointAxis::AngX, 0., 0.1, 0.01)
        .motor_model(JointAxis::AngX, MotorModel::ForceBased)
        // .limits(JointAxis::AngX, [-0.01, PI / 2. + 0.01])
        .limits(JointAxis::AngY, [-0.01, 0.01])
        .limits(JointAxis::AngZ, [-0.01, 0.01])
        .build();

    let shoulder = commands
        .spawn((
            RigidBody::Dynamic,
            GravityScale(0.),
            Collider::cuboid(0.05, 0.05, 0.05),
            ColliderMassProperties::Density(10.0),
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(2., 1.6, 0.))),
            ImpulseJoint::new(upper_torso, TypedJoint::GenericJoint(shoulder_joint)),
            ArmPartMarker::Shoulder,
        ))
        .id();

    let elbow_joint = GenericJointBuilder::new(JointAxesMask::LIN_AXES)
        .local_anchor1(Vec3::new(0.8, 0.0, 0.0))
        .local_anchor2(Vec3::new(0., 0.0, 0.0))
        // .local_axis1(Vec3::Y)
        // .local_axis2(Vec3::Y)
        .coupled_axes(JointAxesMask::LIN_AXES)
        .motor_position(JointAxis::AngZ, 0., 1., 0.1)
        .motor_model(JointAxis::AngZ, MotorModel::ForceBased)
        .limits(JointAxis::AngX, [-0.01, 0.01])
        .limits(JointAxis::AngY, [-0.01, 0.01])
        .limits(JointAxis::AngZ, [PI / 2. - 0.01, PI / 2. + 0.01])
        .build();

    let elbow = commands
        .spawn((
            RigidBody::Dynamic,
            GravityScale(0.),
            Collider::cuboid(0.05, 0.05, 0.05),
            ColliderMassProperties::Density(10.0),
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(3., 1.6, 0.))),
            ImpulseJoint::new(shoulder, TypedJoint::GenericJoint(elbow_joint)),
            ArmPartMarker::Elbow,
        ))
        .id();

    let wrist_joint = GenericJointBuilder::new(JointAxesMask::LIN_AXES)
        .local_anchor1(Vec3::new(0.8, 0.0, 0.0))
        .local_anchor2(Vec3::new(0., 0.0, 0.0))
        // .local_axis1(Vec3::Y)
        // .local_axis2(Vec3::Y)
        .coupled_axes(JointAxesMask::LIN_AXES)
        .motor_position(JointAxis::AngZ, 0., 1., 0.1)
        .motor_model(JointAxis::AngZ, MotorModel::ForceBased)
        .limits(JointAxis::AngX, [-0.01, 0.01])
        .limits(JointAxis::AngY, [-0.01, 0.01])
        .limits(JointAxis::AngZ, [-0.01, 0.01])
        .build();

    let wrist = commands
        .spawn((
            RigidBody::Dynamic,
            GravityScale(0.),
            Collider::cuboid(0.05, 0.05, 0.05),
            ColliderMassProperties::Density(10.0),
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(4., 1.6, 0.))),
            ImpulseJoint::new(elbow, TypedJoint::GenericJoint(wrist_joint)),
        ))
        .id();
}
