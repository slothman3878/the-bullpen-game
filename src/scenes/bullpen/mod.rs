mod events;
mod menu;
mod resources;
mod systems;

use crate::prelude::*;
use events::*;
use menu::*;
use systems::*;

pub(crate) mod prelude {
    pub(crate) use super::*;
    pub(crate) use events::*;
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
            .register_type::<PreviewPassBaseballMarker>()
            .register_type::<PreviewPassBaseballAxisMarker>()
            .register_type::<StrikezoneSpawnRequestMarker>();
    }

    fn add_events(&self, app: &mut App) {
        app.add_event::<PlayerModeSelected>();
    }
}

impl Plugin for BullpenScene {
    fn build(&self, app: &mut App) {
        self.register_type(app);
        self.add_events(app);
        self.configure_set(app);

        app.add_plugins(PitcherPlugin::<BullpenScene> {
            scene: *self,
            render_layers: vec![0],
        })
        .add_plugins(BatterPlugin::<BullpenScene> {
            scene: *self,
            render_layers: vec![0],
        })
        .add_plugins(StrikezonePlugin::<BullpenScene> { scene: *self });

        app.insert_resource(MenuState::default());
        app.insert_resource(ActiveBatterTracker { height: 1.8 });

        app.add_systems(
            OnEnter(Self),
            (
                setup_scene,
                setup_baseball_preview_scene, //
                                              // _spawn_camera.after(setup_scene),
            )
                .chain()
                .in_set(GameScenesSet::OnEnterSet(*self)),
        )
        // resource trackers
        .add_systems(
            Update,
            (
                active_batter_changed, //
            )
                .in_set(GameScenesSet::UpdateSet(*self)),
        )
        // menu systems
        .add_systems(
            Update,
            (
                (
                    params_menu,
                    update_baseball_preview_3d, // baseball_preview_3d,
                )
                    .run_if(menu_visibility_is(true)),
                (
                    toggle_menu_visibility,
                    third_person_camera_lock_status, //
                )
                    .chain()
                    .run_if(input_just_pressed(KeyCode::Escape)),
            )
                .chain()
                .in_set(GameScenesSet::UpdateSet(*self)),
        )
        .add_systems(
            Update,
            spawn_strikezone
                .in_set(GameScenesSet::UpdateSet(*self))
                .in_set(GltfBlueprintsSet::AfterSpawn),
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
        )
        .add_systems(
            Update,
            display_strikezone_panel_intersection_info
                .in_set(UpdateBaseballFlightStateSet::PostUpdate)
                .in_set(GameScenesSet::UpdateSet(*self)),
        )
        .add_systems(
            Update,
            (swap_camera.run_if(input_just_pressed(KeyCode::KeyQ)))
                .in_set(GameScenesSet::UpdateSet(*self)),
        );
    }
}

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub(crate) struct PreviewPassBaseballMarker;

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub(crate) struct PreviewPassBaseballAxisMarker;
