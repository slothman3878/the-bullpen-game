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
    camera_query: Query<&GlobalTransform, With<PitcherCameraMarker>>,
    rapier_context: Res<RapierContext>,
    mut ev_pitch_stage_transition_event: EventWriter<PitchStageTransitionEvents>,
    mut query_pitcher: Query<(Entity, &mut PitchStage, &mut PitcherParams)>, // there must only one pitcher at a time?
) {
    if let Ok(camera_global_transform) = camera_query.get_single() {
        let camera_transform = camera_global_transform.compute_transform();
        let ray_origin = camera_transform.translation;
        let ray_dir = camera_transform.rotation.mul_vec3(-Vec3::Z).normalize();
        let max_toi = f32::INFINITY;
        let query = QueryFilter::new();

        let direction = match rapier_context.cast_ray(ray_origin, ray_dir, max_toi, true, query) {
            Some((_entity, toi)) => {
                let aim_point = ray_origin + ray_dir * toi;
                (aim_point - ray_origin).normalize()
            }
            None => ray_dir,
        };

        for (entity, mut pitch_stage, mut pitcher_params) in query_pitcher.iter_mut() {
            if *pitch_stage != PitchStage::WindUp {
                return;
            }
            ev_pitch_stage_transition_event.send(PitchStageTransitionEvents::KneeUp(entity));
            *pitch_stage = PitchStage::Stride;
            pitcher_params.direction = direction;
            info!("pitcher event: transitioning to stride");
        }
    }
}
pub(crate) fn on_pitch_stage_transition_event(
    mut ev_pitch_stage_transition_event: EventReader<PitchStageTransitionEvents>,
    mut commands: Commands,
    query_pitcher: Query<&PitcherParams>,
    mut query_velocity: Query<&mut Velocity>,
    mut query_body_part: Query<(&mut ImpulseJoint, &Transform), With<PitcherBodyPartMarker>>,
    mut ev_activate_aerodynamics: EventWriter<ActivateAerodynamicsEvent>,
) {
    for ev in ev_pitch_stage_transition_event.read() {
        match ev {
            PitchStageTransitionEvents::KneeUp(pitcher_entity) => {
                if let Ok(pitcher) = query_pitcher.get(*pitcher_entity) {
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
                if let Ok(pitcher) = query_pitcher.get(*pitcher_entity) {
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
                if let Ok(pitcher) = query_pitcher.get(*pitcher_entity) {
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
                if let Ok(pitcher) = query_pitcher.get(*pitcher_entity) {
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
                        // really should be hanlded somewhere else...
                        if let Some(ball) = pitcher.ball {
                            // ugly as fuck, but I want this done
                            if let Ok(mut velocity_mut) = query_velocity.get_mut(ball) {
                                let gyro_pole = GyroPole::default();
                                let spin_efficiency: f32 = 1.;
                                let spin_rate: f32 = 2400.;
                                let seam_y_angle: f32 = PI / 2.;
                                let seam_z_angle: f32 = 0.;
                                let tilt = Tilt::from_hour_mintes(12, 0);
                                let fixed_spin_rate = if spin_rate == 0. { 1. } else { spin_rate };

                                let gyro = match gyro_pole {
                                    GyroPole::Left => spin_efficiency.asin(),
                                    GyroPole::Right => {
                                        std::f32::consts::PI - spin_efficiency.asin()
                                    }
                                };

                                let spin_x_0 =
                                    fixed_spin_rate * (spin_efficiency * tilt.get().sin());
                                let spin_y_0 = fixed_spin_rate * gyro.cos(); // ((1. - spin_efficiency.powi(2)).sqrt());
                                let spin_z_0 =
                                    -fixed_spin_rate * (spin_efficiency * tilt.get().cos());
                                let spin = Vec3::new(
                                    spin_x_0 * RPM_TO_RADS,
                                    spin_y_0 * RPM_TO_RADS, // - RPM_TO_RAD ???
                                    spin_z_0 * RPM_TO_RADS,
                                );

                                velocity_mut.angvel = spin.from_baseball_coord_to_bevy();
                                //
                                // ball launch
                                info!("direction {:?}", pitcher.direction);
                                commands.entity(ball).remove::<ImpulseJoint>().insert(
                                    ExternalImpulse {
                                        impulse: 5.
                                            * (pitcher.direction + Vec3::new(0.05, -0.28, 0.))
                                                .normalize(), // 4. ~ 5.
                                        ..default()
                                    },
                                );
                                //
                                ev_activate_aerodynamics.send(ActivateAerodynamicsEvent {
                                    entity: ball,
                                    seam_y_angle,
                                    seam_z_angle,
                                });
                            }
                        }
                    }
                }
            }
        }
    }
}
