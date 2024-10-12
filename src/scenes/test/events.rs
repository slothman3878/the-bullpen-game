use crate::prelude::*;

#[derive(Debug, Event)]
pub(crate) enum PitchStageTransitionEvents {
    FootContact(Entity),
    PelvisBreak(Entity),
    Release(Entity),
}

pub(crate) fn emit_foot_contact(
    mut ev_pitch_stage_transition_event: EventWriter<PitchStageTransitionEvents>,
    query_pitcher: Query<Entity, With<PitcherParams>>, // there must only one pitcher at a time?
) {
    if let Ok(pitcher_entity) = query_pitcher.get_single() {
        ev_pitch_stage_transition_event
            .send(PitchStageTransitionEvents::FootContact(pitcher_entity));
    }
}

pub(crate) fn on_pitch_stage_transition_event(
    mut ev_pitch_stage_transition_event: EventReader<PitchStageTransitionEvents>,
    mut commands: Commands,
    query_pitcher: Query<&PitcherParams>,
    mut query_body_part: Query<&mut ImpulseJoint, With<BodyPartMarker>>,
) {
    for ev in ev_pitch_stage_transition_event.read() {
        match ev {
            PitchStageTransitionEvents::FootContact(pitcher_entity) => {
                if let Ok(pitcher) = query_pitcher.get(*pitcher_entity) {
                    for (body_part, body_part_entity) in pitcher.body_parts.iter() {
                        if let Ok(mut impulse_joint) = query_body_part.get_mut(*body_part_entity) {
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
            PitchStageTransitionEvents::PelvisBreak(_) => {
                // commands.schedule_on_update(break_system);
            }
            PitchStageTransitionEvents::Release(_) => {
                // commands.schedule_on_update(release_system);
            }
        }
    }
}
