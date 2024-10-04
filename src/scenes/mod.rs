pub(crate) mod bullpen;

pub(crate) mod prelude {
    pub(crate) use super::bullpen;
}

use crate::prelude::*;

#[derive(Debug, Default, Hash, Eq, PartialEq, Clone, Copy, Reflect)]
pub(crate) enum GameScene {
    #[default]
    MainMenu,
    Bullpen,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub(crate) struct GameSceneSet(GameScene);

#[derive(Debug, States, Hash, Eq, PartialEq, Clone)]
pub(crate) enum GameState {
    Loading(GameScene),
    Loaded(GameScene),
}

impl Default for GameState {
    fn default() -> Self {
        Self::Loading(GameScene::default())
    }
}

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub(crate) struct GameSceneMarker(pub GameScene);

// app state(?) loading, etc...

#[derive(Debug)]
pub(crate) struct GameScenesPlugin;

impl Plugin for GameScenesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(GameState::Loading(GameScene::Bullpen));

        app.register_type::<GameSceneMarker>();

        // app.add_systems(PostStartup, (setup_scene, spawn_camera.after(setup_scene)));
        app.configure_sets(
            OnEnter(GameState::Loading(GameScene::Bullpen)),
            GameSceneSet(GameScene::Bullpen),
        )
        .configure_sets(
            Update,
            GameSceneSet(GameScene::Bullpen)
                .run_if(in_state(GameState::Loaded(GameScene::Bullpen))),
        );

        // app.add_systems(
        //     Update,
        //     spawn_ball
        //         .run_if(input_just_released(KeyCode::KeyR))
        //         .in_set(AeroActivationSet::PreActivation),
        // );
    }
}
