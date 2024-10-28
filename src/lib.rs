mod ball_flight_state;
mod common;
mod components;
mod errors;
mod events;
mod resources;
mod systems;

pub mod prelude {
    pub use super::{
        ball_flight_state::BaseballFlightState, components::*, constants::*, errors::*, events::*,
        utils::*, BaseballFlightPlugin, GyroPole, Tilt,
    };
}

use crate::systems::*;

pub(crate) use crate::resources::*;
pub(crate) use ball_flight_state::*;
pub(crate) use bevy::{math::*, prelude::*}; // glam
pub(crate) use bevy_rapier3d::prelude::*; // nalgebra
pub(crate) use common::*;
pub(crate) use constants::*;
pub(crate) use errors::*;
pub(crate) use events::*;
pub(crate) use utils::*;

pub struct BaseballFlightPlugin {
    pub ssw_on: bool,
    pub magnus_on: bool,
    pub drag_on: bool,
}

impl Plugin for BaseballFlightPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ActivateAerodynamicsEvent>()
            .add_event::<PostActivateAerodynamicsEvent>()
            .add_event::<DisableAerodynamicsEvent>();

        // app.register_type::<BaseballFlightState>();

        app.insert_resource(BaseballPluginConfig {
            ssw_on: self.ssw_on,
            magnus_on: self.magnus_on,
            drag_on: self.drag_on,
            ..default()
        });

        app.configure_sets(
            Update,
            (
                AeroActivationSet::PreActivation,
                AeroActivationSet::Activation,
                AeroActivationSet::PostActivation,
            )
                .chain(),
        )
        .configure_sets(
            Update,
            (
                UpdateBaseballFlightStateSet::PreUpdate,
                UpdateBaseballFlightStateSet::Update,
                UpdateBaseballFlightStateSet::PostUpdate,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (activate_aerodynamics,)
                .chain()
                .in_set(AeroActivationSet::Activation),
        );

        app.configure_sets(
            Update,
            (
                AeroDeactivationSet::PreDeactivation,
                AeroDeactivationSet::Deactivation,
                AeroDeactivationSet::PostDeactivation,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (disable_aerodynamics,)
                .chain()
                .in_set(AeroDeactivationSet::Deactivation),
        );

        app.add_systems(
            Update,
            (
                // _apply_physics_option_1,
                // _apply_physics_option_2,
                _apply_physics_option_3
            )
                .in_set(UpdateBaseballFlightStateSet::Update),
        );
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum AeroActivationSet {
    PreActivation,
    Activation,
    PostActivation,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum AeroDeactivationSet {
    PreDeactivation,
    Deactivation,
    PostDeactivation,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum UpdateBaseballFlightStateSet {
    PreUpdate,
    Update,
    PostUpdate,
}

#[derive(Debug, Clone, Copy, Reflect, PartialEq, Eq)]
pub enum GyroPole {
    Right,
    Left,
}

impl Default for GyroPole {
    fn default() -> Self {
        Self::Right
    }
}

#[derive(Debug, Reflect, Copy, Clone)]
pub struct Tilt(f32);
impl Tilt {
    pub fn from_hour_mintes(h: i8, m: i8) -> Result<Self> {
        if h > 12 || h <= 0 {
            return Err(BaseballFlightError::InvalidInput(
                "hr should be within the range of 1 and 12".into(),
            ));
        }
        if m < 0 || m > 59 {
            return Err(BaseballFlightError::InvalidInput(
                "min should be within the range of 0 and 59".into(),
            ));
        }
        let rad_hrs = (h - 3) as f32 * PI_32 / 6.;
        let rad_mins = m as f32 * PI_32 / 360.;
        Ok(Self(rad_hrs + rad_mins))
    }

    pub fn to_hour_minutes(&self) -> (i8, i8) {
        let total_hours = (self.0 * 6.0 / PI_32) + 3.0;
        let hrs = total_hours.floor() as i8;
        let mins = ((total_hours.fract() * 60.0).round() as i8) % 60;
        ((hrs - 1) % 12 + 1, mins)
    }

    pub fn get(&self) -> f32 {
        self.0
    }
}
