use crate::*;

#[derive(Debug, Clone, Bundle)]
pub struct BaseballFlightBundle {
    pub state: BaseballFlightState,
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub mass: ColliderMassProperties,
    pub gravity_scale: GravityScale,
    // pub transform: Transform,
    // pub linear_velocity: LinearVelocity,
}

impl Default for BaseballFlightBundle {
    fn default() -> Self {
        let collider = Collider::ball(RADIUS);
        Self {
            state: BaseballFlightState::default(),
            collider,
            rigid_body: RigidBody::Dynamic,
            mass: ColliderMassProperties::Mass(MASS),
            gravity_scale: GravityScale(0.0),
            // transform: Transform::default(),
            // linear_velocity: LinearVelocity::default(),
        }
    }
}
