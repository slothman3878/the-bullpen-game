use crate::prelude::*;

#[derive(Debug, Event)]
pub(crate) enum PitchStageTransitionEvents {
    KneeUp(Entity),
    FootContact(Entity),
    MaxER(Entity),
    // Release(Entity),
    MaxIR(Entity),
}

pub fn _release_ball(
    query_pitcher: Query<&PitcherParams>,
    mut commands: Commands,
    mut ev_activate_aerodynamics: EventWriter<ActivateAerodynamicsEvent>,
) {
    for pitcher in query_pitcher.iter() {
        if let Some(ball) = pitcher.ball {
            // ball launch
            commands
                .entity(ball)
                .remove::<ImpulseJoint>()
                .insert(ExternalImpulse {
                    impulse: 4. * (pitcher.direction).normalize(), // 4. ~ 5.
                    ..default()
                })
                .insert((Restitution {
                    coefficient: 0.546,
                    combine_rule: CoefficientCombineRule::Min,
                },));
            //
            ev_activate_aerodynamics.send(ActivateAerodynamicsEvent {
                entity: ball,
                seam_y_angle: 0.,
                seam_z_angle: std::f32::consts::PI / 2.,
            });
        }
    }
}

pub(crate) fn emit_knee_up(
    mut ev_pitch_stage_transition_event: EventWriter<PitchStageTransitionEvents>,
    mut query_pitcher: Query<(Entity, &mut PitchStage), With<PitcherParams>>, // there must only one pitcher at a time?
) {
    for (entity, mut pitch_stage) in query_pitcher.iter_mut() {
        if *pitch_stage != PitchStage::WindUp {
            return;
        }
        ev_pitch_stage_transition_event.send(PitchStageTransitionEvents::KneeUp(entity));
        *pitch_stage = PitchStage::Stride;
        info!("pitcher event: transitioning to stride");
    }
}
pub(crate) fn on_pitch_stage_transition_event(
    mut ev_pitch_stage_transition_event: EventReader<PitchStageTransitionEvents>,
    mut commands: Commands,
    mut query_pitcher: Query<&PitcherParams>,
    mut query_body_part: Query<(&mut ImpulseJoint, &Transform), With<PitcherBodyPartMarker>>,
    mut ev_activate_aerodynamics: EventWriter<ActivateAerodynamicsEvent>,
) {
    for ev in ev_pitch_stage_transition_event.read() {
        match ev {
            PitchStageTransitionEvents::KneeUp(pitcher_entity) => {
                if let Ok(pitcher) = query_pitcher.get_mut(*pitcher_entity) {
                    for (body_part, body_part_entity) in pitcher.body_parts.iter() {
                        if let Ok((mut impulse_joint, _transform)) =
                            query_body_part.get_mut(*body_part_entity)
                        {
                            pitcher.on_knee_up(
                                &mut commands,
                                body_part,
                                *body_part_entity,
                                &mut impulse_joint,
                            );
                            if let Some(balance_weight) = pitcher.balance_weight {
                                commands
                                    .entity(balance_weight)
                                    .insert(ExternalForce::at_point(
                                        10000. * pitcher.direction, // Vec3::Z,
                                        Vec3::new(0., pitcher.leg_length, 0.),
                                        Vec3::new(0., pitcher.leg_length, 0.),
                                    ));
                            }
                        }
                    }
                }
            }
            PitchStageTransitionEvents::FootContact(pitcher_entity) => {
                if let Ok(pitcher) = query_pitcher.get_mut(*pitcher_entity) {
                    for (body_part, body_part_entity) in pitcher.body_parts.iter() {
                        if let Ok((mut impulse_joint, _transform)) =
                            query_body_part.get_mut(*body_part_entity)
                        {
                            pitcher.on_foot_contact(
                                &mut commands,
                                body_part,
                                *body_part_entity,
                                &mut impulse_joint,
                            );
                            if let Some(balance_weight) = pitcher.balance_weight {
                                commands
                                    .entity(balance_weight)
                                    .remove::<RigidBody>()
                                    .insert(RigidBody::Fixed);
                            }
                        }
                    }
                }
            }
            PitchStageTransitionEvents::MaxER(pitcher_entity) => {
                if let Ok(pitcher) = query_pitcher.get_mut(*pitcher_entity) {
                    for (body_part, body_part_entity) in pitcher.body_parts.iter() {
                        if let Ok((mut impulse_joint, transform)) =
                            query_body_part.get_mut(*body_part_entity)
                        {
                            pitcher.on_max_er(
                                &mut commands,
                                body_part,
                                *body_part_entity,
                                &mut impulse_joint,
                                transform.translation,
                            );
                        }
                    }
                }
            }
            PitchStageTransitionEvents::MaxIR(pitcher_entity) => {
                if let Ok(pitcher) = query_pitcher.get_mut(*pitcher_entity) {
                    for (body_part, body_part_entity) in pitcher.body_parts.iter() {
                        if let Ok((mut impulse_joint, _)) =
                            query_body_part.get_mut(*body_part_entity)
                        {
                            pitcher.on_max_ir(
                                &mut commands,
                                body_part,
                                *body_part_entity,
                                &mut impulse_joint,
                            );
                        }
                        if let Some(ball) = pitcher.ball {
                            // ball launch
                            commands.entity(ball).remove::<ImpulseJoint>().insert(
                                ExternalImpulse {
                                    impulse: 4. * (pitcher.direction).normalize(), // 4. ~ 5.
                                    ..default()
                                },
                            );
                            //
                            ev_activate_aerodynamics.send(ActivateAerodynamicsEvent {
                                entity: ball,
                                seam_y_angle: 0.,
                                seam_z_angle: std::f32::consts::PI / 2.,
                            });
                        }
                    }
                }
            }
        }
    }
}
