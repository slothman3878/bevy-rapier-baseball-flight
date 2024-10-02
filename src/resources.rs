use crate::*;

#[derive(Debug, Resource)]
pub(crate) struct BaseballPluginConfig {
    pub ssw_on: bool,
    pub magnus_on: bool,
    pub drag_on: bool,
    pub ssw: SeamShiftedWake,
}

// probably should be a resource
/// seam shifted wake parameters
#[derive(Debug, Copy, Clone)]
pub(crate) struct SeamShiftedWake {
    // this number effects how much the separation location will change based on the spin rate. Bigger, Move shift allows for the moving the effectiveness of the seams forwards or backwards.
    pub seam_shift_factor: f64,
    // in rad
    pub angle_of_activation: f64,
    // move activation area. is usually positive
    pub activation_shift: f64,
    // in rad
    pub separated_flow_range: f64,
}

impl Default for SeamShiftedWake {
    fn default() -> Self {
        Self {
            seam_shift_factor: 1.5,
            angle_of_activation: 5. * PI_64 / 180.,
            activation_shift: 0.21,
            separated_flow_range: 35. * PI_64 / 180.,
        }
    }
}

impl SeamShiftedWake {
    pub(crate) fn get_activation_region(&self) -> (DVec3, DVec3) {
        let acceptable_range = SEAM_DIAMETER * 1.1;
        let acceptable_thickness = SEAM_DIAMETER / 2. * (2. * self.angle_of_activation).sin();

        let x_max = 0.5 * acceptable_range;
        let x_min = -0.5 * acceptable_range;
        let y_max = acceptable_thickness + self.activation_shift;
        let y_min = -acceptable_thickness + self.activation_shift; // note that the -y axis is the direction of travel
        let z_max = 0.5 * acceptable_range;
        let z_min = -0.5 * acceptable_range;

        (
            DVec3::new(x_max, y_max, z_max),
            DVec3::new(x_min, y_min, z_min),
        )
    }
}

impl Default for BaseballPluginConfig {
    fn default() -> Self {
        Self {
            ssw_on: true,
            magnus_on: true,
            drag_on: true,
            ssw: SeamShiftedWake::default(),
        }
    }
}
