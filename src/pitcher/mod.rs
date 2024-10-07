use crate::prelude::*;

mod components;
mod events;
mod resources;
mod systems;

use components::*;
use events::*;
use resources::*;
use systems::*;

pub(crate) mod prelude {
    pub(crate) use super::*;
}

#[derive(Debug)]
pub(crate) struct PitcherPlugin<T: GameScene> {
    pub scene: T,
}

impl<T: GameScene> Plugin for PitcherPlugin<T> {
    fn build(&self, app: &mut App) {
        app.register_type::<PitcherMarker>()
            .register_type::<PitcherCameraMarker>()
            .register_type::<PitcherCameraTargetMarker>();

        // app.insert_resource(SelectedPitchParameters(PitchParams::demo()));

        app.add_systems(
            Update,
            setup_camera
                .in_set(GameScenesSet::UpdateSet(self.scene.clone()))
                .in_set(GltfBlueprintsSet::AfterSpawn),
        )
        .add_systems(
            Update,
            start_pitch
                .run_if(input_just_released(KeyCode::KeyK))
                .in_set(GameScenesSet::UpdateSet(self.scene.clone())),
        );
    }
}

pub fn setup_camera(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query_pitcher: Query<Entity, (With<PitcherCameraTargetMarker>, With<Transform>)>,
) {
    match query_pitcher.get_single() {
        Ok(pitcher) => {
            commands
                .entity(pitcher)
                .insert(ThirdPersonCameraTarget)
                .remove::<PitcherCameraTargetMarker>();

            commands
                .spawn((
                    ThirdPersonCamera {
                        aim_speed: 5.0,
                        cursor_lock_toggle_enabled: true,
                        offset_enabled: true,
                        offset: Offset::new(0.5, 1.7),
                        zoom: Zoom::new(8., 20.),
                        cursor_lock_key: KeyCode::Escape,
                        ..default()
                    },
                    Camera3dBundle {
                        projection: blenvy::Projection::Perspective(PerspectiveProjection {
                            fov: 10.0_f32.to_radians(),
                            ..default()
                        }),
                        camera: Camera {
                            is_active: true,
                            order: 2,
                            ..default()
                        },
                        ..default()
                    },
                    PitcherCameraMarker,
                    Name::new("pitcher camera"),
                    InheritedVisibility::VISIBLE,
                ))
                .with_children(|parent| {
                    parent.spawn((PbrBundle {
                        mesh: meshes.add(Sphere::new(0.0005)).into(), // default 0.075
                        material: materials.add(Color::srgb(0.1, 0.1, 0.1)),
                        transform: Transform::from_xyz(0., 0., -0.5),
                        ..default()
                    },));
                });
        }
        Err(_) => {
            // info!("No pitcher found. No camera attached.");
        }
    }
}
