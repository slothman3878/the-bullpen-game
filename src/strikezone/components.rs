use crate::prelude::*;

/// Marker component for the homeplate to attach the strikezone to
#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub(crate) struct StrikezoneHomeplateMarker;

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub(crate) struct BallStrikezoneCollisionMarker;

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub(crate) enum StrikezonePanel {
    Front {
        dimensions: Vec2,
        collision_point: Vec3,
        updated: bool,
    },
    Back {
        dimensions: Vec2,
        collision_point: Vec3,
        updated: bool,
    },
}

impl StrikezonePanel {
    pub fn is_updated(&self) -> bool {
        match self {
            Self::Front { updated, .. } => *updated,
            Self::Back { updated, .. } => *updated,
        }
    }

    pub fn collision_point(&self) -> Vec3 {
        match self {
            Self::Front {
                collision_point, ..
            } => *collision_point,
            Self::Back {
                collision_point, ..
            } => *collision_point,
        }
    }

    pub fn dimensions(&self) -> Vec2 {
        match self {
            Self::Front { dimensions, .. } => *dimensions,
            Self::Back { dimensions, .. } => *dimensions,
        }
    }

    pub fn set_collision_point(&mut self, point: Vec3) -> Result<(), Error> {
        if self.is_updated() {
            return Err(Error::GenericError("panel already updated".to_string()));
        }
        match self {
            Self::Front {
                collision_point,
                updated,
                ..
            } => {
                *collision_point = point;
                *updated = true;
            }
            Self::Back {
                collision_point,
                updated,
                ..
            } => {
                *collision_point = point;
                *updated = true;
            }
        }
        Ok(())
    }

    pub fn clear(&mut self, new_dimensions: Vec2) {
        match self {
            Self::Front {
                dimensions,
                collision_point,
                updated,
            } => {
                *dimensions = new_dimensions;
                *collision_point = Vec3::default();
                *updated = false;
            }
            Self::Back {
                dimensions,
                collision_point,
                updated,
            } => {
                *dimensions = new_dimensions;
                *collision_point = Vec3::default();
                *updated = false;
            }
        }
    }

    pub fn new_back(new_dimensions: Vec2) -> Self {
        Self::Back {
            dimensions: new_dimensions,
            collision_point: Vec3::default(),
            updated: false,
        }
    }

    pub fn new_front(new_dimensions: Vec2) -> Self {
        Self::Front {
            dimensions: new_dimensions,
            collision_point: Vec3::default(),
            updated: false,
        }
    }
}

#[derive(Debug, Bundle)]
pub(crate) struct StrikezonePanelBundle {
    pub sensor: Sensor,
    pub panel: StrikezonePanel,
    pub collider: Collider,
    pub transform: TransformBundle,
    pub active_events: ActiveEvents,
    // pub rigidbody: RigidBody,
}
impl StrikezonePanelBundle {
    pub fn new_front(half_height: f32, pos_y: f32) -> Self {
        let pos = Vec3::new(
            0.,
            pos_y, //
            DEFAULT_FRONT_PANEL_POS_Z,
        );

        Self {
            sensor: Sensor,
            panel: StrikezonePanel::new_front(Vec2::new(DEFAULT_LENGTH_HALF, half_height)),
            collider: Collider::cuboid(DEFAULT_LENGTH_HALF, half_height, 0.001),
            transform: TransformBundle::from_transform(Transform::from_translation(pos)),
            active_events: ActiveEvents::COLLISION_EVENTS,
            // rigidbody: RigidBody::Fixed,
        }
    }

    pub fn new_back(half_height: f32, pos_y: f32) -> Self {
        let pos = Vec3::new(
            0.,
            pos_y - DEFAULT_BACK_PANEL_Y_DIFF,
            DEFAULT_BACK_PANEL_POS_Z,
        );

        Self {
            sensor: Sensor,
            panel: StrikezonePanel::new_back(Vec2::new(DEFAULT_LENGTH_HALF, half_height)),
            collider: Collider::cuboid(DEFAULT_LENGTH_HALF, half_height, 0.001),
            transform: TransformBundle::from_transform(Transform::from_translation(pos)),
            active_events: ActiveEvents::COLLISION_EVENTS,
            // rigidbody: RigidBody::Fixed,
        }
    }
}
