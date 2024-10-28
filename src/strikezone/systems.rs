use crate::prelude::*;

pub(crate) fn spawn_strikezone_system(
    mut commands: Commands,
    mut ev_spawn: EventReader<SpawnStrikezone>,
    query: Query<(Entity, &Transform), With<StrikezoneHomeplateMarker>>,
) {
    for ev in ev_spawn.read() {
        let batter_height = ev.batter_height;
        for (home_plate_entity, _) in query.iter() {
            info!("home_plate_entity: {}", home_plate_entity);
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

                    spawn_front_panel(children, half_height, pos_y);
                    spawn_back_panel(children, half_height, pos_y);
                });
        }
    }
}

// pub(crate) fn draw_strikezone_squares()

fn spawn_front_panel(children: &mut ChildBuilder, half_height: f32, pos_y: f32) {
    let pos = Vec3::new(
        0.,
        pos_y, //
        DEFAULT_FRONT_PANEL_POS_Z,
    );

    children.spawn((
        Sensor,
        Collider::cuboid(DEFAULT_LENGTH_HALF, half_height, 0.0005),
        TransformBundle::from_transform(Transform::from_translation(pos)),
    ));
}

fn spawn_back_panel(children: &mut ChildBuilder, half_height: f32, pos_y: f32) {
    let pos = Vec3::new(
        0.,
        pos_y - DEFAULT_BACK_PANEL_Y_DIFF,
        DEFAULT_BACK_PANEL_POS_Z,
    );

    children.spawn((
        Sensor,
        Collider::cuboid(DEFAULT_LENGTH_HALF, half_height, 0.0005),
        TransformBundle::from_transform(Transform::from_translation(pos)),
    ));
}
