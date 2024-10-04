use crate::*;

// option 1 - update transform
pub(crate) fn _apply_physics_option_1(
    time_fixed: Res<Time<Fixed>>,
    rapier_config: Res<RapierConfiguration>,
    baseball_plugin_config: Res<BaseballPluginConfig>,
    mut query_baseball: Query<(&mut BaseballFlightState, &mut Transform)>,
) {
    let delta_t = get_delta_t(&time_fixed, &rapier_config);
    for (mut state, mut transform) in &mut query_baseball {
        if state.active {
            state.update_state(&baseball_plugin_config, delta_t);
            transform.translation = state.translation.as_vec3().from_baseball_coord_to_bevy();
        }
    }
}

// option 2 - update velocity
pub(crate) fn _apply_physics_option_2(
    time_fixed: Res<Time<Fixed>>,
    rapier_config: Res<RapierConfiguration>,
    baseball_plugin_config: Res<BaseballPluginConfig>,
    mut query_baseball: Query<(&mut BaseballFlightState, &Transform, &mut Velocity)>,
) {
    let delta_t = get_delta_t(&time_fixed, &rapier_config);
    for (mut state, transform, mut velo) in &mut query_baseball {
        if state.active {
            let new_velo = state._update_state_and_get_velo(
                &baseball_plugin_config,
                transform
                    .translation
                    .from_bevy_to_baseball_coord()
                    .as_dvec3(),
                delta_t,
            );
            velo.linvel = new_velo.from_baseball_coord_to_bevy().as_vec3();
        }
    }
}

// preferred
// option 3 - apply external force
pub(crate) fn _apply_physics_option_3(
    time_fixed: Res<Time<Fixed>>,
    rapier_config: Res<RapierConfiguration>,
    baseball_plugin_config: Res<BaseballPluginConfig>,
    mut query_baseball: Query<(
        &mut BaseballFlightState,
        &Transform,
        &Velocity,
        &mut ExternalForce,
    )>,
) {
    let delta_t = get_delta_t(&time_fixed, &rapier_config);
    for (mut state, transform, velo, mut force) in &mut query_baseball {
        if state.active {
            let a = state.update_state_and_get_acceleration(
                &baseball_plugin_config,
                transform
                    .translation
                    .from_bevy_to_baseball_coord()
                    .as_dvec3(),
                velo.linvel.from_bevy_to_baseball_coord().as_dvec3(),
                delta_t,
            );
            force.force = a.from_baseball_coord_to_bevy().as_vec3() * MASS;
        } else {
            // info!("inactive aerodynamics");
        }
    }
}

fn get_delta_t(time_fixed: &Res<Time<Fixed>>, rapier_config: &Res<RapierConfiguration>) -> f64 {
    match rapier_config.timestep_mode {
        TimestepMode::Variable { time_scale, .. } => {
            time_scale as f64 * time_fixed.delta_seconds_f64()
        }
        TimestepMode::Fixed { dt, .. } => dt as f64,
        _ => 1.,
    }
}

pub(crate) fn activate_aerodynamics(
    mut ball_physics_query: Query<(
        &mut BaseballFlightState,
        &mut ExternalForce,
        &Transform,
        &Velocity,
    )>,
    mut ev_activate_aerodynamics_event: EventReader<ActivateAerodynamicsEvent>,
    mut ev_post_activate_aerodynamics_event: EventWriter<PostActivateAerodynamicsEvent>,
) {
    for ev in ev_activate_aerodynamics_event.read() {
        // info!("activate aerodynamics {:?}", ev.entity);
        if let Ok((mut state, mut force, transform, velo)) = ball_physics_query.get_mut(ev.entity) {
            // info!("query aerodynamics");
            if !state.active {
                // just in case
                force.force = Vec3::ZERO;
                //
                *state = BaseballFlightState::from_params(
                    transform
                        .translation
                        .from_bevy_to_baseball_coord()
                        .as_dvec3(),
                    velo.linvel.from_bevy_to_baseball_coord().as_dvec3(),
                    velo.angvel.from_bevy_to_baseball_coord().as_dvec3(),
                    ev.seam_y_angle,
                    ev.seam_z_angle,
                );
                //
                ev_post_activate_aerodynamics_event.send(PostActivateAerodynamicsEvent(ev.entity));
            }
        }
    }
}

pub(crate) fn disable_aerodynamics(
    mut ball_physics_query: Query<(
        &mut BaseballFlightState,
        &mut ExternalForce,
        &mut GravityScale,
    )>,
    mut ev_disable_aerodynamics_event: EventReader<DisableAerodynamicsEvent>,
) {
    for ev in ev_disable_aerodynamics_event.read() {
        if let Ok((mut ball, mut force, mut gravity_scale)) = ball_physics_query.get_mut(ev.0) {
            if ball.active {
                ball.deactivate();
                force.force = Vec3::ZERO;
                gravity_scale.0 = 1.;
            }
        }
    }
}
