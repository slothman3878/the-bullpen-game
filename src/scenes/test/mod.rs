mod components;
mod events;
mod systems;

use systems::*;

use bevy::pbr::CascadeShadowConfigBuilder;

use crate::prelude::*;

pub(crate) mod prelude {
    pub(crate) use super::components::*;
    pub(crate) use super::events::*;
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
        app.register_type::<GameSceneMarker<Self>>()
            .register_type::<BodyPartMarker>();
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
                spawn_floor,
                spawn_arms,
                // spawn_camera.after(setup_scene),
            )
                .chain()
                .in_set(GameScenesSet::OnEnterSet(*self)),
        )
        .add_systems(
            Update,
            (
                // max_er.run_if(input_just_released(KeyCode::KeyR)),
                // release.run_if(input_just_released(MouseButton::Left)),
                emit_knee_up.run_if(input_just_released(KeyCode::KeyQ)),
                emit_foot_contact.run_if(input_just_released(KeyCode::KeyR)),
                // mark_velo.run_if(input_just_released(KeyCode::KeyM)), // push_shoulder.run_if(input_pressed(MouseButton::Left)),
            )
                .chain()
                .in_set(GameScenesSet::UpdateSet(*self)),
        )
        .add_systems(
            Update,
            (
                pelvic_rotation_tracker,
                on_pitch_stage_transition_event,
                wrist_z_pos_tracker,
                mark_velo,
                // apply_force_shoulder,
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

pub(crate) fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((
            RigidBody::Fixed,
            Collider::cuboid(100., 0.01, 100.),
            TransformBundle::from_transform(Transform::from_translation(Vec3::ZERO)),
            InheritedVisibility::VISIBLE,
        ))
        .with_children(|children| {
            children.spawn((PbrBundle {
                mesh: meshes.add(Cuboid::new(100., 0.01, 100.)).into(), // d
                material: materials.add(Color::WHITE),
                ..default()
            },));
        });
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
