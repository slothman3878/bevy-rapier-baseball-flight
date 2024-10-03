use crate::*;

// option 1 - update transform
pub(crate) fn _apply_physics_option_1(
    time_fixed: Res<Time<Fixed>>,
    rapier_config: Res<RapierConfiguration>,
    baseball_plugin_config: Res<BaseballPluginConfig>,
    mut query_baseball: Query<(&mut BaseballFlightState, &mut Transform)>,
) {
    let timescale = match rapier_config.timestep_mode {
        TimestepMode::Variable { time_scale, .. } => time_scale,
        _ => 1.,
    };
    let delta_t = time_fixed.delta_seconds_f64() * timescale as f64;
    for (mut state, mut transform) in &mut query_baseball {
        state.update_state(&baseball_plugin_config, delta_t);
        transform.translation = state.translation.as_vec3().from_baseball_coord_to_bevy();
    }
}

// option 2 - update velocity
pub(crate) fn _apply_physics_option_2(
    time_fixed: Res<Time<Fixed>>,
    rapier_config: Res<RapierConfiguration>,
    baseball_plugin_config: Res<BaseballPluginConfig>,
    mut query_baseball: Query<(
        &mut BaseballFlightState,
        &Transform,
        &mut Velocity,
        &mut GravityScale,
    )>,
) {
    let timescale = match rapier_config.timestep_mode {
        TimestepMode::Variable { time_scale, .. } => time_scale,
        _ => 1.,
    };
    let delta_t = time_fixed.delta_seconds_f64() * timescale as f64;
    for (mut state, transform, mut velo, mut gravity_scale) in &mut query_baseball {
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
        } else {
            gravity_scale.0 = 1.;
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
    let delta_t = match rapier_config.timestep_mode {
        TimestepMode::Variable { time_scale, .. } => {
            time_scale as f64 * time_fixed.delta_seconds_f64()
        }
        TimestepMode::Fixed { dt, .. } => dt as f64,
        _ => 1.,
    };
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
            info!("transform {:?}", transform.translation);
        } else {
            // info!("inactive aerodynamics");
        }
    }
}

pub(crate) fn activate_aerodynamics(
    mut ball_physics_query: Query<(
        &mut BaseballFlightState,
        &mut ExternalForce,
        &mut GravityScale,
        &Transform,
        &Velocity,
    )>,
    mut ev_activate_aerodynamics_event: EventReader<ActivateAerodynamicsEvent>,
    mut ev_post_activate_aerodynamics_event: EventWriter<PostActivateAerodynamicsEvent>,
) {
    for ev in ev_activate_aerodynamics_event.read() {
        if let Ok((mut state, mut force, mut gravity_scale, transform, velo)) =
            ball_physics_query.get_mut(ev.entity)
        {
            if !state.active {
                // just in case
                force.force = Vec3::ZERO;
                gravity_scale.0 = 0.;
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
