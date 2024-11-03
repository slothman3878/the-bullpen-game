use crate::prelude::*;

#[derive(Debug, Deref, Resource)]
pub(crate) struct BaseballPreviewImage(Handle<Image>);

impl BaseballPreviewImage {
    pub(crate) fn new(image_handle: Handle<Image>) -> Self {
        Self(image_handle)
    }
}

#[derive(Debug, Resource, Default)]
pub(crate) struct ActiveBatterTracker {
    /// batter's height in meters
    pub height: f32,
}

pub(crate) fn active_batter_changed(
    active_batter_tracker: Res<ActiveBatterTracker>,
    mut ev_redraw_strikezone: EventWriter<RedrawStrikezone>,
) {
    if active_batter_tracker.is_changed() {
        ev_redraw_strikezone.send(RedrawStrikezone {
            batter_height: active_batter_tracker.height,
        });
    }
}

#[derive(Debug, Default, States, Hash, Eq, PartialEq, Clone, Copy)]
pub(crate) enum BullpenSceneGameMode {
    #[default]
    Pitcher,
    Batter,
}
