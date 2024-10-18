use crate::prelude::*;

mod components;
mod events;
mod systems;

use events::*;
use systems::*;

pub(crate) mod prelude {
    pub(crate) use super::*;
    pub(crate) use components::*;
}

#[derive(Debug)]
pub(crate) struct PitcherMechanics<T: GameScene> {
    pub scene: T,
}

impl<T: GameScene> Plugin for PitcherMechanics<T> {
    fn build(&self, app: &mut App) {
        app.register_type::<PitcherBodyPartMarker>();

        app.add_event::<PitchStageTransitionEvents>();

        app.add_systems(
            OnEnter(self.scene.clone()),
            (spawn_pitcher_mechanics,)
                .chain()
                .in_set(GameScenesSet::OnEnterSet(self.scene.clone())),
        )
        .add_systems(
            Update,
            (
                emit_knee_up.run_if(input_just_released(KeyCode::KeyQ)),
                emit_foot_contact,
            )
                .chain()
                .in_set(GameScenesSet::UpdateSet(self.scene.clone())),
        )
        .add_systems(
            Update,
            (
                pelvic_rotation_tracker,
                on_pitch_stage_transition_event,
                wrist_z_pos_tracker,
            )
                .chain()
                .in_set(GameScenesSet::UpdateSet(self.scene.clone())),
        );
    }
}
