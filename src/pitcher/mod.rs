use crate::prelude::*;

mod components;
mod events;
mod resources;
mod systems;

use components::*;
use events::*;
use resources::*;

pub(crate) mod prelude {
    pub(crate) use super::*;
    pub(crate) use components::*;
    pub(crate) use resources::*;
    pub(crate) const PITCH_DEFAULT_STARTING_POINT_RIGHTY: Vec3 = Vec3::new(0.48, 1.82, 16.764);
    pub(crate) const PITCH_DEFAULT_STARTING_POINT_LEFTY: Vec3 = Vec3::new(-0.48, 1.82, 16.764);
}

#[derive(Debug)]
pub(crate) struct PitcherPlugin<T: GameScene> {
    pub scene: T,
    pub render_layers: Vec<usize>,
}

impl<T: GameScene> Plugin for PitcherPlugin<T> {
    fn build(&self, app: &mut App) {
        app.register_type::<PitcherMarker>()
            .register_type::<PitcherCameraMarker>()
            .register_type::<PitcherCameraTargetMarker>();

        app.add_event::<BaseballLaunchEvent>();

        app.insert_resource(PitcherPluginConfig {
            render_layers: self.render_layers.clone(),
        })
        .insert_resource(SelectedPitchParameters(PitchParams {
            gyro_pole: GyroPole::default(),
            spin_efficiency: 1.,
            speed: 90., // 90. * MPH_TO_FTS,
            spin_rate: 2000.,
            seam_y_angle: 0.,
            seam_z_angle: std::f32::consts::PI / 2.,
            tilt: Tilt::from_hour_mintes(12, 0).expect("invalid initial tilt params".into()),
            // starting_point: Vec3::new(0.48, 1.82, 16.764),
            pitching_arm: PitchingArm::Righty,
            direction: Vec3::ZERO,
        }));
        // app.add_systems(OnEnter(self.scene.clone()), spawn_arms);

        app.add_systems(OnEnter(self.scene.clone()), spawn_pitcher);

        app.add_systems(
            Update,
            setup_camera
                .in_set(GameScenesSet::UpdateSet(self.scene.clone()))
                .in_set(GltfBlueprintsSet::AfterSpawn),
        );
    }
}

pub fn spawn_pitcher(mut commands: Commands) {
    commands.spawn((
        TransformBundle::from_transform(Transform::from_translation(Vec3::new(0., 0.15, 18.44))),
        PitcherCameraTargetMarker,
    ));
}

pub fn setup_camera(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    pitcher_plugin_config: Res<PitcherPluginConfig>,
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
                        offset: Offset::new(0., 1.7),
                        zoom: Zoom::new(8., 15.),
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
                            // order: 2,
                            ..default()
                        },
                        ..default()
                    },
                    RenderLayers::from_layers(&pitcher_plugin_config.render_layers),
                    // RenderLayers::layer(0),
                    PitcherCameraMarker,
                    Name::new("pitcher camera"),
                    InheritedVisibility::VISIBLE,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        PbrBundle {
                            mesh: meshes.add(Sphere::new(0.0005)).into(), // default 0.075
                            material: materials.add(Color::srgb(0.1, 0.1, 0.1)),
                            transform: Transform::from_xyz(0., 0., -0.5),
                            ..default()
                        },
                        RenderLayers::from_layers(&pitcher_plugin_config.render_layers),
                        // RenderLayers::layer(0),
                    ));
                });
        }
        Err(_) => {
            // info!("No pitcher found. No camera attached.");
        }
    }
}
