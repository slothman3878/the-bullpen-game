use crate::prelude::*;

#[derive(Debug, Event)]
pub(crate) struct SpawnStrikezone {
    pub batter_height: f32,
}

#[derive(Debug, Event)]
pub(crate) struct RedrawStrikezone {
    pub batter_height: f32,
}

#[derive(Debug, Event)]
pub(crate) struct RecordStrikezoneCollision {
    pub panel: Entity,
    pub collision_point: Vec3,
}
