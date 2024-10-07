use crate::prelude::*;

#[derive(Debug, Component, Reflect, Clone, Copy)]
#[reflect(Component)]
pub(crate) struct PitcherMarker;

#[derive(Debug, Component, Reflect, Clone, Copy)]
#[reflect(Component)]
pub(crate) struct PitcherCameraMarker;

#[derive(Debug, Component, Reflect, Clone, Copy)]
#[reflect(Component)]
pub(crate) struct PitcherCameraTargetMarker;
