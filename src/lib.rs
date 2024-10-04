mod ball_flight_state;
mod common;
mod components;
mod events;
mod resources;
mod systems;

pub mod prelude {
    pub use super::{
        components::*, constants::*, events::*, utils::*, BaseballFlightPlugin, GyroPole, Tilt,
    };
}

use crate::systems::*;

pub(crate) use crate::resources::*;
pub(crate) use ball_flight_state::*;
pub(crate) use bevy::{math::*, prelude::*}; // glam
pub(crate) use bevy_rapier3d::prelude::*; // nalgebra
pub(crate) use common::*;
pub(crate) use constants::*;
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

        app.register_type::<BaseballFlightState>();

        app.insert_resource(BaseballPluginConfig {
            ssw_on: self.ssw_on,
            magnus_on: self.magnus_on,
            drag_on: self.drag_on,
            ..default()
        });

        // app.add_systems(Update, _apply_physics_option_1);
        // app.add_systems(Update, _apply_physics_option_2);
        app.add_systems(Update, _apply_physics_option_3);

        app.add_systems(Update, activate_aerodynamics);
        app.add_systems(Update, disable_aerodynamics);
    }
}

#[derive(Debug, Clone, Copy, Reflect)]
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
    pub fn from_hour_mintes(h: i8, m: i8) -> Self {
        assert!(h <= 12 && h > 0);
        let rad_hrs = (h - 3) as f32 * PI_32 / 6.;
        let rad_mins = m as f32 * PI_32 / 360.;
        Self(rad_hrs + rad_mins)
    }

    pub fn get(&self) -> f32 {
        self.0
    }
}
