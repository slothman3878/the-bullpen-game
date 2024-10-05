extern crate proc_macro;

pub(crate) mod bullpen;

pub(crate) mod prelude {
    pub(crate) use super::bullpen::*;
    pub(crate) use super::*;
}

use crate::prelude::*;

use std::fmt::Debug;
use std::hash::Hash;

pub(crate) trait GameScene: States // 'static + Send + Sync + Clone + PartialEq + Eq + Hash + Debug
{
    fn register_type(&self, app: &mut App);
    fn configure_set(&self, app: &mut App);
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub(crate) enum GameScenesSet<T: GameScene> {
    OnEnterSet(T),
    UpdateSet(T),
    OnExitSet(T),
}

#[derive(Debug, States, Hash, Eq, PartialEq, Clone)]
pub(crate) enum SceneState {
    Loading,
    Loaded,
}

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub(crate) struct GameSceneMarker<T: GameScene>(pub T);

// app state(?) loading, etc...

#[derive(Debug)]
pub(crate) struct GameScenePlugin;

impl Plugin for GameScenePlugin {
    fn build(&self, app: &mut App) {
        // default scene to start from
        app.insert_state(BullpenScene);

        app.add_plugins(BullpenScene);
    }
}
