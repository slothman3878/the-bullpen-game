mod components;
mod events;
mod systems;

use crate::prelude::*;

pub(crate) mod prelude {
    // // dhx
    // pub(crate) const DEFAULT_WIDTH_HALF: f32 = 0.118;
    // dhz
    pub(crate) const DEFAULT_LENGTH_HALF: f32 = 0.2359;
    // dyz
    pub(crate) const DEFAULT_HEIGHT_TOP_PERCENTAGE: f32 = 0.5635;
    pub(crate) const DEFAULT_HEIGHT_BOTTOM_PERCENTAGE: f32 = 0.2764;

    // replace with appropriate y values
    pub(crate) const DEFAULT_FRONT_PANEL_POS_Z: f32 = 0.4318;
    pub(crate) const DEFAULT_BACK_PANEL_POS_Z: f32 = 0.2159;
    pub(crate) const DEFAULT_BACK_PANEL_Y_DIFF: f32 = 0.015;

    pub(crate) use super::*;
    pub(crate) use components::*;
    pub(crate) use events::*;
    pub(crate) use systems::*;
}

#[derive(Debug)]
pub(crate) struct StrikezonePlugin<T: GameScene> {
    pub scene: T,
}

impl<T: GameScene> Plugin for StrikezonePlugin<T> {
    fn build(&self, app: &mut App) {
        app.register_type::<StrikezoneHomeplateMarker>();

        app.add_event::<SpawnStrikezone>()
            .add_event::<RedrawStrikezone>()
            .add_event::<RecordStrikezoneCollision>();

        app.add_systems(
            Update,
            (
                spawn_strikezone_system,
                update_strikezone_panel_system,
                record_strikezone_collision_system,
                draw_panels,
            )
                .in_set(GameScenesSet::UpdateSet(self.scene.clone())),
        );
    }
}
