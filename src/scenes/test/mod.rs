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

pub(crate) fn spawn_arms(mut commands: Commands) {
    let upper_torso = commands
        .spawn((
            RigidBody::Dynamic,
            GravityScale(0.),
            Collider::cuboid(0.5, 0.1, 0.1),
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(0., 1.6, 0.))),
            // Velocity {
            //     linvel: Vec3::new(0., 0., 1.),
            //     ..default()
            // },
        ))
        .id();

    let shoulder_joint = SphericalJointBuilder::new()
        .local_anchor1(Vec3::new(1., 0.0, 0.0))
        .local_anchor2(Vec3::new(0., 0.0, 0.0))
        .motor_position(JointAxis::AngZ, PI, 0.1, 0.);
    let shoulder = commands
        .spawn((
            RigidBody::Dynamic,
            GravityScale(0.),
            Collider::cuboid(0.05, 0.05, 0.05),
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(1., 1.6, 0.))),
            ImpulseJoint::new(upper_torso, shoulder_joint),
        ))
        .id();

    let elbow_joint = SphericalJointBuilder::new()
        .local_anchor1(Vec3::new(1., 0., 0.))
        .local_anchor2(Vec3::ZERO)
        .motor_position(JointAxis::AngZ, PI, 0.1, 0.);
    let elbow = commands
        .spawn((
            RigidBody::Dynamic,
            GravityScale(0.),
            Collider::cuboid(0.05, 0.05, 0.05),
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(2., 1.6, 0.))),
            ImpulseJoint::new(shoulder, elbow_joint),
        ))
        .id();
    // commands
    //     .entity(parent_entity)
    //     .insert(ExternalImpulse::at_point(
    //         Vec3::new(0., 0., 0.1),
    //         Vec3::new(0., 1.6, 0.),
    //         Vec3::new(0., 1.6, 0.),
    //     ));
}
