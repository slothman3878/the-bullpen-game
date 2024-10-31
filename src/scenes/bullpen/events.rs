use crate::prelude::*;

#[derive(Debug, Event)]
pub struct PlayerModeSelected(pub(crate) PlayerMode);

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum PlayerMode {
    Pitcher,
    Batter,
    // FreeCam,
}
