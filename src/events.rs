use crate::*;

#[derive(Debug, Clone, Event)]
pub struct ActivateAerodynamicsEvent {
    pub entity: Entity,
    pub seam_y_angle: f32,
    pub seam_z_angle: f32,
    //
    pub record_times: Vec<f64>,
    //
    pub strikezone_panels_z: (f32, f32),
}

#[derive(Debug, Clone, Copy, Event)]
pub struct PostActivateAerodynamicsEvent(pub Entity);

#[derive(Debug, Clone, Copy, Event)]
pub struct DisableAerodynamicsEvent(pub Entity);
