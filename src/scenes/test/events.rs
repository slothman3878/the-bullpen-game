use crate::{pitcher, prelude::*};

#[derive(Debug, Event)]
pub(crate) enum PitchStageTransitionEvents {
    FootContact(Entity),
    MaxER(Entity),
    Release(Entity),
    MaxIR(Entity),
}

pub(crate) fn emit_foot_contact(
    mut ev_pitch_stage_transition_event: EventWriter<PitchStageTransitionEvents>,
    mut query_pitcher: Query<(Entity, &mut PitchStage), With<PitcherParams>>, // there must only one pitcher at a time?
) {
    for (entity, mut pitch_stage) in query_pitcher.iter_mut() {
        if *pitch_stage != PitchStage::Stride {
            return;
        }
        ev_pitch_stage_transition_event.send(PitchStageTransitionEvents::FootContact(entity));
        *pitch_stage = PitchStage::ArmCocking;
        info!("transitioning to arm cocking");
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
            pitcher_params.body_parts.get(&BodyPartMarker::Pelvis),
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
    query_transform: Query<&Transform, With<BodyPartMarker>>,
    mut query_pitcher: Query<(Entity, &PitcherParams, &mut PitchStage)>,
) {
    for (entity, pitcher_params, mut pitch_stage) in query_pitcher.iter_mut() {
        if *pitch_stage != PitchStage::ArmAcceleration {
            return;
        }
        if let Some(wrist) = pitcher_params.body_parts.get(&BodyPartMarker::Wrist) {
            if let Ok(transform) = query_transform.get(*wrist) {
                if transform.translation.z > 0. {
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
    mut query_body_part: Query<(&mut ImpulseJoint, &Transform), With<BodyPartMarker>>,
) {
    for ev in ev_pitch_stage_transition_event.read() {
        match ev {
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
            PitchStageTransitionEvents::Release(_) => {
                // commands.schedule_on_update(release_system);
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
                    }
                }
            }
        }
    }
}
