mod systems;

use systems::*;

use crate::prelude::*;

pub(crate) mod prelude {
    pub(crate) use super::*;
}

// bullpen scene
#[derive(Debug, Reflect, States, Hash, Eq, PartialEq, Clone, Copy)]
pub(crate) struct BullpenScene;

impl GameScene for BullpenScene {
    fn configure_set(&self, app: &mut App) {
        app.configure_sets(
            OnEnter(Self),
            GameScenesSet::OnEnterSet(Self).run_if(in_state(Self)),
        )
        .configure_sets(
            Update,
            GameScenesSet::UpdateSet(Self).run_if(in_state(Self)),
        )
        .configure_sets(
            OnExit(Self),
            GameScenesSet::OnExitSet(Self).run_if(in_state(Self)),
        );
    }

    fn register_type(&self, app: &mut App) {
        app.register_type::<GameSceneMarker<Self>>();
    }
}

impl Plugin for BullpenScene {
    fn build(&self, app: &mut App) {
        self.register_type(app);
        self.configure_set(app);

        app.add_systems(
            OnEnter(Self),
            (setup_scene, spawn_camera.after(setup_scene)).in_set(GameScenesSet::OnEnterSet(Self)),
        )
        .add_systems(
            Update,
            (spawn_ball
                .run_if(input_just_released(KeyCode::KeyR))
                .in_set(AeroActivationSet::PreActivation))
            .in_set(GameScenesSet::UpdateSet(Self)),
        );
    }
}
