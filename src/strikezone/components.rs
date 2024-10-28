use crate::prelude::*;

/// Marker component for the homeplate to attach the strikezone to
#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub(crate) struct StrikezoneHomeplateMarker;

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub(crate) enum StrikezonePanel {
    Front(Vec3, bool),
    Back(Vec3, bool),
}
impl StrikezonePanel {
    pub fn is_updated(&self) -> bool {
        match self {
            Self::Front(_, ref updated) => *updated,
            Self::Back(_, ref updated) => *updated,
        }
    }

    pub fn collision_point(&self) -> Vec3 {
        match self {
            Self::Front(ref point, _) => *point,
            Self::Back(ref point, _) => *point,
        }
    }

    pub fn set_collision_point(&mut self, point: Vec3) {
        match self {
            Self::Front(ref mut front, ref mut updated) => {
                *front = point;
                *updated = true;
            }
            Self::Back(ref mut back, ref mut updated) => {
                *back = point;
                *updated = true;
            }
        }
    }

    pub fn clear(&mut self) {
        match self {
            Self::Front(ref mut front, ref mut updated) => {
                *front = Vec3::default();
                *updated = false;
            }
            Self::Back(ref mut back, ref mut updated) => {
                *back = Vec3::default();
                *updated = false;
            }
        }
    }

    pub fn new_back() -> Self {
        Self::Back(Vec3::default(), false)
    }
    pub fn new_front() -> Self {
        Self::Front(Vec3::default(), false)
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
            panel: StrikezonePanel::new_front(),
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
            panel: StrikezonePanel::new_back(),
            collider: Collider::cuboid(DEFAULT_LENGTH_HALF, half_height, 0.001),
            transform: TransformBundle::from_transform(Transform::from_translation(pos)),
            active_events: ActiveEvents::COLLISION_EVENTS,
            // rigidbody: RigidBody::Fixed,
        }
    }
}
