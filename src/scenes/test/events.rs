use crate::prelude::*;

#[derive(Debug, Event)]
pub(crate) enum PitchStageTransitionEvents {
    FootContact(Entity),
    PelvisBreak(Entity),
    Release(Entity),
}

fn on_pitch_stage_transition_event(
    mut ev_pitch_stage_transition_event: EventReader<PitchStageTransitionEvents>,
    mut commands: Commands,
    mut query_pitcher: Query<Entity, With<PitcherMarker>>,
    mut query_body_part: Query<(Entity, &mut ImpulseJoint, &BodyPartMarker)>,
) {
    for ev in ev_pitch_stage_transition_event.read() {
        match ev {
            PitchStageTransitionEvents::FootContact(entity) => {
                // need to get children of entity
                if let Ok((entity, mut impulse_joint, arm_part)) = query_body_part.get(*entity) {}
            }
            PitchStageTransitionEvents::PelvisBreak(entity) => {
                // commands.schedule_on_update(break_system);
            }
            PitchStageTransitionEvents::Release(entity) => {
                // commands.schedule_on_update(release_system);
            }
        }
    }
}
