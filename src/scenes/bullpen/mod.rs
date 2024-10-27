mod menu;
mod resources;
mod systems;

use systems::*;

use crate::prelude::*;
use menu::*;

pub(crate) mod prelude {
    pub(crate) use super::*;
    pub(crate) use resources::*;
}

// bullpen scene
#[derive(Debug, Reflect, States, Hash, Eq, PartialEq, Clone, Copy)]
pub(crate) struct BullpenScene;

impl GameScene for BullpenScene {
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
            .register_type::<PreviewPassBaseballMarker>();
    }
}

impl Plugin for BullpenScene {
    fn build(&self, app: &mut App) {
        self.register_type(app);
        self.configure_set(app);

        app.add_plugins(PitcherPlugin::<BullpenScene> {
            scene: *self,
            render_layers: vec![0],
        });

        app.insert_resource(MenuVisibility(false));

        app.add_systems(
            OnEnter(Self),
            (
                setup_scene,
                setup_baseball_preview_scene, // spawn_camera.after(setup_scene),
            )
                .in_set(GameScenesSet::OnEnterSet(*self)),
        )
        .add_systems(
            Update,
            (
                (
                    params_menu,
                    update_baseball_preview_3d, // baseball_preview_3d,
                )
                    .run_if(menu_visibility_is(true)),
                (toggle_menu_visibility, third_person_camera_lock_status)
                    .chain()
                    .run_if(input_just_pressed(KeyCode::Escape)),
            )
                .chain()
                .in_set(GameScenesSet::UpdateSet(*self)),
        )
        .add_systems(
            Update,
            (spawn_ball
                .run_if(input_just_pressed(MouseButton::Right))
                .in_set(AeroActivationSet::PreActivation))
            .in_set(GameScenesSet::UpdateSet(*self)),
        )
        .add_systems(
            Update,
            (launch_ball
                .run_if(input_just_released(MouseButton::Right))
                .in_set(AeroActivationSet::PreActivation))
            .in_set(GameScenesSet::UpdateSet(*self)),
        )
        .add_systems(
            Update,
            (despawn_ball
                .run_if(input_just_released(KeyCode::KeyR))
                .in_set(AeroActivationSet::PostActivation))
            .in_set(GameScenesSet::UpdateSet(*self)),
        );
    }
}

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub(crate) struct PreviewPassBaseballMarker;
