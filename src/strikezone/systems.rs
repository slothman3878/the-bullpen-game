use crate::prelude::*;

pub(crate) fn spawn_strikezone_system(
    mut commands: Commands,
    mut ev_spawn: EventReader<SpawnStrikezone>,
    query: Query<(Entity, &Transform), With<StrikezoneHomeplateMarker>>,
) {
    for ev in ev_spawn.read() {
        let batter_height = ev.batter_height;
        for (home_plate_entity, _) in query.iter() {
            commands
                .entity(home_plate_entity)
                .with_children(|children| {
                    let half_height = (DEFAULT_HEIGHT_TOP_PERCENTAGE
                        - DEFAULT_HEIGHT_BOTTOM_PERCENTAGE)
                        * 0.5
                        * batter_height;
                    let pos_y = (DEFAULT_HEIGHT_TOP_PERCENTAGE + DEFAULT_HEIGHT_BOTTOM_PERCENTAGE)
                        * 0.5
                        * batter_height;

                    children.spawn(StrikezonePanelBundle::new_front(half_height, pos_y));
                    children.spawn(StrikezonePanelBundle::new_back(half_height, pos_y));
                });
        }
    }
}

// honestly there is only one strikezone panel
pub(crate) fn update_strikezone_panel_system(
    mut query: Query<(&mut Transform, &mut Collider, &mut StrikezonePanel)>,
    mut ev_redraw: EventReader<RedrawStrikezone>,
) {
    for ev in ev_redraw.read() {
        let batter_height = ev.batter_height;
        for (mut transform, mut collider, mut panel) in query.iter_mut() {
            panel.clear();
            //
            let half_height = (DEFAULT_HEIGHT_TOP_PERCENTAGE - DEFAULT_HEIGHT_BOTTOM_PERCENTAGE)
                * 0.5
                * batter_height;
            let pos_y = (DEFAULT_HEIGHT_TOP_PERCENTAGE + DEFAULT_HEIGHT_BOTTOM_PERCENTAGE)
                * 0.5
                * batter_height;
            let new_bundle = match *panel {
                StrikezonePanel::Front(_, _) => {
                    StrikezonePanelBundle::new_front(half_height, pos_y)
                }
                StrikezonePanel::Back(_, _) => {
                    StrikezonePanelBundle::new_back(half_height, pos_y) //
                }
            };
            *collider = new_bundle.collider;
            *transform = new_bundle.transform.local;
        }
    }
}

// pub(crate) fn draw_strikezone_squares()

fn _spawn_front_panel(children: &mut ChildBuilder, half_height: f32, pos_y: f32) {
    let pos = Vec3::new(
        0.,
        pos_y, //
        DEFAULT_FRONT_PANEL_POS_Z,
    );

    children.spawn((
        Sensor,
        Collider::cuboid(DEFAULT_LENGTH_HALF, half_height, 0.0005),
        StrikezonePanel::new_front(),
        TransformBundle::from_transform(Transform::from_translation(pos)),
    ));
}

fn _spawn_back_panel(children: &mut ChildBuilder, half_height: f32, pos_y: f32) {
    let pos = Vec3::new(
        0.,
        pos_y - DEFAULT_BACK_PANEL_Y_DIFF,
        DEFAULT_BACK_PANEL_POS_Z,
    );

    children.spawn((
        Sensor,
        Collider::cuboid(DEFAULT_LENGTH_HALF, half_height, 0.0005),
        StrikezonePanel::new_back(),
        TransformBundle::from_transform(Transform::from_translation(pos)),
    ));
}
