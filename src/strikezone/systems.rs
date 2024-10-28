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
    mut commands: Commands,
    query_collision_record: Query<Entity, With<BallStrikezoneCollisionMarker>>,
    mut query_strikezone: Query<(&mut Transform, &mut Collider, &mut StrikezonePanel)>,
    mut ev_redraw: EventReader<RedrawStrikezone>,
) {
    for ev in ev_redraw.read() {
        let batter_height = ev.batter_height;
        for (mut transform, mut collider, mut panel) in query_strikezone.iter_mut() {
            //
            let half_height = (DEFAULT_HEIGHT_TOP_PERCENTAGE - DEFAULT_HEIGHT_BOTTOM_PERCENTAGE)
                * 0.5
                * batter_height;
            let pos_y = (DEFAULT_HEIGHT_TOP_PERCENTAGE + DEFAULT_HEIGHT_BOTTOM_PERCENTAGE)
                * 0.5
                * batter_height;
            panel.clear(Vec2::new(DEFAULT_LENGTH_HALF, half_height));
            // clear collision records
            for record_entity in query_collision_record.iter() {
                commands.entity(record_entity).despawn_recursive();
            }
            let new_bundle = match *panel {
                StrikezonePanel::Front { .. } => {
                    StrikezonePanelBundle::new_front(half_height, pos_y)
                }
                StrikezonePanel::Back { .. } => {
                    StrikezonePanelBundle::new_back(half_height, pos_y) //
                }
            };
            *collider = new_bundle.collider;
            *transform = new_bundle.transform.local;
        }
    }
}

pub(crate) fn record_strikezone_collision_system(
    mut commands: Commands,
    mut ev_record: EventReader<RecordStrikezoneCollision>,
    mut query_strikezone: Query<(&mut StrikezonePanel, &GlobalTransform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for ev in ev_record.read() {
        if let Ok((mut panel, _)) = query_strikezone.get_mut(ev.panel) {
            if let Ok(_) = panel.set_collision_point(ev.collision_point) {
                info!("collision point updated");
                // unfortunately, the z pos of the records is not the correct z pos at collision
                // not sure how to make this work
                commands
                    .spawn((
                        BallStrikezoneCollisionMarker,
                        InheritedVisibility::VISIBLE,
                        TransformBundle::from(Transform::from_translation(ev.collision_point)),
                    ))
                    .with_children(|parent| {
                        let color = match *panel {
                            StrikezonePanel::Front { .. } => Color::srgba(0.1, 0.1, 0.9, 0.7),
                            StrikezonePanel::Back { .. } => Color::srgba(0.9, 0.4, 0.1, 0.7),
                        };

                        parent.spawn(PbrBundle {
                            mesh: meshes.add(Sphere::new(0.03)).into(), // default 0.075
                            material: materials.add(color),
                            ..default()
                        });
                    });
            }
        }
    }
}

pub(crate) fn draw_panels(
    mut gizmos: Gizmos,
    query_strikezone: Query<(&Transform, &StrikezonePanel)>,
) {
    for (transform, panel) in query_strikezone.iter() {
        match panel {
            StrikezonePanel::Front { dimensions, .. } => {
                gizmos.rect(
                    transform.translation,
                    transform.rotation,
                    *dimensions * 2.,
                    Color::srgba(0.1, 0.1, 0.9, 1.),
                );
            }
            StrikezonePanel::Back { dimensions, .. } => {
                gizmos.rect(
                    transform.translation,
                    transform.rotation,
                    *dimensions * 2.,
                    Color::srgba(0.9, 0.4, 0.9, 1.),
                );
            }
        }
    }
}
