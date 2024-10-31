use crate::prelude::*;

pub(crate) mod prelude {
    pub(crate) use super::*;
}

#[derive(Debug)]
pub(crate) struct BatterPlugin<T: GameScene> {
    pub scene: T,
    pub render_layers: Vec<usize>,
}

impl<T: GameScene> Plugin for BatterPlugin<T> {
    fn build(&self, app: &mut App) {
        app.register_type::<BatterCameraMarker>();

        app.add_systems(OnEnter(self.scene.clone()), setup_batter_camera);
    }
}

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub(crate) struct BatterCameraMarker;

pub(crate) fn setup_batter_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("batter cam"),
        Camera3dBundle {
            camera: Camera {
                is_active: false,
                order: 1,
                ..default()
            },
            transform: Transform::from_xyz(-0.0, 1.0, -3.4)
                .looking_at(Vec3::new(0., 1.2, 0.216), Vec3::Y),
            ..default()
        },
        BatterCameraMarker,
    ));
}
