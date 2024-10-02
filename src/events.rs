use crate::*;

#[derive(Debug, Clone, Copy, Event)]
pub struct ActivateAerodynamicsEvent {
    pub entity: Entity,
    pub seam_y_angle: f32,
    pub seam_z_angle: f32,
}

#[derive(Debug, Clone, Copy, Event)]
pub struct PostActivateAerodynamicsEvent(pub Entity);

#[derive(Debug, Clone, Copy, Event)]
pub struct DisableAerodynamicsEvent(pub Entity);
