use crate::*;

pub(crate) mod constants {
    pub(crate) use std::f32::consts::PI as PI_32;
    pub(crate) use std::f64::consts::PI as PI_64;

    pub const M_TO_FEET: f32 = 3.28084;
    pub const KG_TO_IBS: f32 = 2.20462;

    pub const KMH_TO_MPH: f32 = 0.621371;

    pub const RPM_TO_RADS: f32 = 2. * PI_32 / 60.;
    pub const MPH_TO_FTS: f32 = 1.467;

    pub(crate) const MASS: f32 = 0.145; // in kg
    pub(crate) const RADIUS: f32 = 0.037; // in m

    // pub(crate) const SEAM_R: f32 = (2. + 15. / 16.) / 2.; // in m

    // in pounds and ft/s
    pub(crate) const RHO: f64 = 0.074;
    // const CIRC: f64 = 9.125 / 12.;
    pub(crate) const T_STEP: f64 = 0.001;
    pub(crate) const N_SEAMS: usize = 108;
    pub(crate) const DIAMETER: f64 = (2. + 15. / 16.) / 12.;
    const MASS_OZ: f64 = 0.3203125;
    const AREA: f64 = 0.25 * PI_64 * DIAMETER * DIAMETER;
    pub(crate) const C_0: f64 = 0.5 * RHO * AREA / MASS_OZ;
    pub(crate) const CD_CONST: f64 = 0.33; // drag coefficient
    pub(crate) const C_SEAMS: f64 = 0.02; // The coefficient of Seams "Cseams" is the essentially the Lift coeficient
                                          // per seam per length away from the origin.

    pub(crate) const SEAM_DIAMETER: f64 = 2. + 15. / 16.;
    pub(crate) const SPIN_DECAY: f64 = 10000.; // natural spin decay should be a large value
}

pub(crate) mod utils {
    use super::*;

    pub fn swap_coordinates_vec3(vec: &Vec3) -> Vec3 {
        Vec3::new(-vec.x, vec.z, vec.y) // maybe should consider changing the units as well?
    }

    pub fn swap_coordinates_dvec3(vec: &DVec3) -> DVec3 {
        DVec3::new(-vec.x, vec.z, vec.y) // maybe should consider changing the units as well?
    }

    pub fn kg_to_pound(weight: f32) -> f32 {
        weight * KG_TO_IBS
    }

    pub fn pound_to_kg(weight: f32) -> f32 {
        weight / KG_TO_IBS
    }

    pub trait BaseballCoordinateSystem {
        fn from_bevy_to_baseball_coord(&self) -> Self;
        fn from_baseball_coord_to_bevy(&self) -> Self;
    }

    impl BaseballCoordinateSystem for Vec3 {
        fn from_bevy_to_baseball_coord(&self) -> Self {
            // convert to baseball coordinate system
            swap_coordinates_vec3(self) * M_TO_FEET
        }

        fn from_baseball_coord_to_bevy(&self) -> Self {
            // convert to bevy coordinate system
            swap_coordinates_vec3(self) / M_TO_FEET
        }
    }

    impl BaseballCoordinateSystem for DVec3 {
        fn from_bevy_to_baseball_coord(&self) -> Self {
            // convert to baseball coordinate system
            swap_coordinates_dvec3(self) * (M_TO_FEET as f64)
        }

        fn from_baseball_coord_to_bevy(&self) -> Self {
            // convert to bevy coordinate system
            swap_coordinates_dvec3(self) / (M_TO_FEET as f64)
        }
    }

    pub fn get_angular_velocity_from_parameters(
        tilt: Tilt,
        spin_efficiency: f32,
        spin_rate: f32,
        gyro_pole: GyroPole,
    ) -> Vec3 {
        let fixed_spin_rate = if spin_rate == 0. { 1. } else { spin_rate };

        let gyro = match gyro_pole {
            GyroPole::Left => spin_efficiency.asin(),
            GyroPole::Right => std::f32::consts::PI - spin_efficiency.asin(),
        };

        let spin_x_0 = fixed_spin_rate * (spin_efficiency * tilt.get().sin());
        let spin_y_0 = fixed_spin_rate * gyro.cos(); // ((1. - spin_efficiency.powi(2)).sqrt());
        let spin_z_0 = -fixed_spin_rate * (spin_efficiency * tilt.get().cos());
        Vec3::new(
            spin_x_0 * RPM_TO_RADS,
            spin_y_0 * RPM_TO_RADS, // - RPM_TO_RAD ???
            spin_z_0 * RPM_TO_RADS,
        )
    }
}
