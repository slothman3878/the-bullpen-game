use crate::prelude::*;

#[derive(Debug, Event)]
pub(crate) enum PitchStageTransitionEvents {
    KneeUp(Entity),
    FootContact(Entity),
    MaxER(Entity),
    // Release(Entity),
    MaxIR(Entity),
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
        info!("transitioning to stride");
    }
}

pub(crate) fn emit_foot_contact(
    mut ev_pitch_stage_transition_event: EventWriter<PitchStageTransitionEvents>,
    query_global_transform: Query<&Transform, With<PitcherBodyPartMarker>>,
    mut query_pitcher: Query<(Entity, &PitcherParams, &mut PitchStage)>, // there must only one pitcher at a time?
) {
    for (entity, pitcher_params, mut pitch_stage) in query_pitcher.iter_mut() {
        if *pitch_stage != PitchStage::Stride {
            return;
        }
        if let Some(core_entity) = pitcher_params.body_parts.get(&PitcherBodyPartMarker::Core) {
            if let Ok(transform) = query_global_transform.get(*core_entity) {
                // let transform = global_transform.compute_transform();
                info!(
                    "{} v {:?}",
                    (pitcher_params.leg_length - pitcher_params.torso_drop),
                    transform.translation.y
                );
                if transform.translation.y < (pitcher_params.leg_length - pitcher_params.torso_drop)
                {
                    ev_pitch_stage_transition_event
                        .send(PitchStageTransitionEvents::FootContact(entity));
                    *pitch_stage = PitchStage::ArmCocking;
                    info!("transitioning to arm cocking");
                }
            }
        }
    }
}

pub(crate) fn pelvic_rotation_tracker(
    mut ev_pitch_stage_transition_event: EventWriter<PitchStageTransitionEvents>,
    rapier_context: Res<RapierContext>,
    mut query_pitcher: Query<(Entity, &PitcherParams, &mut PitchStage)>,
) {
    for (entity, pitcher_params, mut pitch_stage) in query_pitcher.iter_mut() {
        if *pitch_stage != PitchStage::ArmCocking {
            return;
        }
        if let (Some(pelvis), Some(pelvic_break_sensor)) = (
            pitcher_params
                .body_parts
                .get(&PitcherBodyPartMarker::Pelvis),
            pitcher_params.pelvic_break,
        ) {
            if Some(true) == rapier_context.intersection_pair(*pelvis, pelvic_break_sensor) {
                ev_pitch_stage_transition_event.send(PitchStageTransitionEvents::MaxER(entity));
                *pitch_stage = PitchStage::ArmAcceleration;
                info!("transitioning to arm acceleration");
            }
        }
    }
}

pub(crate) fn wrist_z_pos_tracker(
    mut ev_pitch_stage_transition_event: EventWriter<PitchStageTransitionEvents>,
    query_transform: Query<&Transform, With<PitcherBodyPartMarker>>,
    mut query_pitcher: Query<(Entity, &PitcherParams, &mut PitchStage)>,
) {
    for (entity, pitcher_params, mut pitch_stage) in query_pitcher.iter_mut() {
        if *pitch_stage != PitchStage::ArmAcceleration {
            return;
        }
        if let Some(wrist) = pitcher_params.body_parts.get(&PitcherBodyPartMarker::Wrist) {
            if let Ok(transform) = query_transform.get(*wrist) {
                let rotation = pitcher_params.direction.angle_between(Vec3::Z);
                info!("rotation: {:?}", rotation);
                if Quat::from_rotation_y(-rotation)
                    .mul_vec3(transform.translation)
                    .z
                    > 0.
                {
                    ev_pitch_stage_transition_event.send(PitchStageTransitionEvents::MaxIR(entity));
                    *pitch_stage = PitchStage::ArmDeceleration;
                    info!("transitioning to arm deceleration");
                }
            }
        }
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
            }
        }
    }
}
