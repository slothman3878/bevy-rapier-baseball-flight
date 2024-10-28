use crate::*;

#[derive(Debug, Component, Clone, Default)]
pub struct BaseballFlightState {
    pub(crate) translation: DVec3,
    pub(crate) v: DVec3,
    pub(crate) spin: DVec3,
    pub(crate) seams: Vec<DVec3>,
    pub(crate) time_elapsed: f64,
    //
    pub(crate) active: bool,
    // for recording purposes
    pub(crate) record_on: bool,
    pub(crate) record_times: Vec<f64>,
    pub(crate) record_positions: Vec<DVec3>,
    // strikezone recording purposes (front, back)
    pub(crate) strikezone_panels_y: (f64, f64),
    pub(crate) pos_at_strikezone_panels_y: (DVec3, DVec3),
}

impl BaseballFlightState {
    pub fn get_pos_at_strikezone_panels_z(&self) -> (Vec3, Vec3) {
        // record strikezone position
        let (pos_front, pos_back) = self.pos_at_strikezone_panels_y;
        (
            pos_front.from_baseball_coord_to_bevy().as_vec3(),
            pos_back.from_baseball_coord_to_bevy().as_vec3(),
        )
    }

    pub(crate) fn deactivate(&mut self) {
        self.active = false;
        self.time_elapsed = 0.;
    }

    pub(crate) fn from_params(
        // position in baseball coord
        translation_: DVec3,
        // velocity in baseball coord
        velocity_: DVec3,
        // spin in rads
        spin_: DVec3,
        // in rad
        seam_y_angle_: f32,
        // in rad
        seam_z_angle_: f32,
        // other parameters...
        record_times_: Vec<f64>,
        //
        strikezone_panels_y: (f64, f64),
    ) -> Self {
        // Return owned value instead of reference
        let translation = translation_;
        let v = velocity_;
        let spin = spin_;
        let seam_y_angle = seam_y_angle_ as f64;
        let seam_z_angle = seam_z_angle_ as f64;

        let seams = (0..N_SEAMS)
            .map(|i| {
                let alpha =
                    (PI_64 * 2.) * (f64::from((i % N_SEAMS) as i16) / f64::from(N_SEAMS as i16));
                let x = (1. / 13.) * (9. * f64::cos(alpha) - 4. * f64::cos(3. * alpha));
                let y = (1. / 13.) * (9. * f64::sin(alpha) + 4. * f64::sin(3. * alpha));
                let z = (12. / 13.) * f64::cos(2. * alpha);
                DVec3::new(x, y, z) * (SEAM_DIAMETER / 2.)
            })
            .collect::<Vec<_>>();

        let seams_adjsuted = seams
            .iter()
            .map(|point| {
                // X axis of seams space should be the axis of rotation
                DQuat::from_rotation_arc(DVec3::X, spin.normalize()).mul_vec3(
                    DQuat::from_rotation_z(-seam_z_angle).mul_vec3(
                        DQuat::from_rotation_y(seam_y_angle).mul_vec3(
                            DQuat::from_rotation_y(PI_64 / 2.)
                                .mul_vec3(DQuat::from_rotation_x(-PI_64 / 2.).mul_vec3(*point)),
                        ),
                    ),
                )
            })
            .collect::<Vec<_>>();

        let default_record_positions = record_times_
            .iter()
            .map(|_: &f64| DVec3::ZERO)
            .collect::<Vec<_>>();

        Self {
            // Return the value directly, not a reference
            translation,
            v,
            spin,
            seams: seams_adjsuted,
            time_elapsed: 0.,
            active: true,
            record_on: false,
            record_times: record_times_,
            record_positions: default_record_positions,
            strikezone_panels_y,
            pos_at_strikezone_panels_y: (DVec3::ZERO, DVec3::ZERO),
        }
    }

    // option 3
    pub(crate) fn update_state_and_get_acceleration(
        &mut self,
        config: &BaseballPluginConfig,
        translation: DVec3,
        velocity: DVec3,
        delta_t: f64,
    ) -> DVec3 {
        self.translation = translation;
        self.v = velocity;

        self.update_state(config, delta_t);
        let distance = self.translation - translation;

        (self.v * self.v - velocity * velocity) / (2. * distance)
    }

    // option 2
    pub(crate) fn _update_state_and_get_velo(
        &mut self,
        config: &BaseballPluginConfig,
        translation: DVec3,
        delta_t: f64,
    ) -> DVec3 {
        self.translation = translation;

        self.update_state(config, delta_t);

        (self.translation - translation) / delta_t
    }

    // option 1
    pub(crate) fn update_state(&mut self, config: &BaseballPluginConfig, delta_t: f64) {
        let iterations = (delta_t * 1000.).floor() as usize;

        for _ in 0..iterations {
            // rotate seams
            self.seams = self
                .seams
                .iter()
                .map(|point| {
                    // in seam space, the seams are rotating around the local x axis
                    DQuat::from_axis_angle(self.spin.normalize(), self.spin.length() * T_STEP)
                        .mul_vec3(*point)
                })
                .collect::<Vec<_>>();

            let active_seams = self.find_ssw_seams(&config.ssw);

            let a = self.rk4(config, &active_seams);

            self.time_elapsed += T_STEP;

            self.v += DVec3::new(a.x, a.y, a.z - 32.2) * T_STEP;
            self.translation += self.v * T_STEP;

            // record position
            if self.record_on {
                if let Some(index) = self
                    .record_times
                    .iter()
                    .position(|&t| t > self.time_elapsed)
                {
                    self.record_positions[index] = self.translation;
                }
            }
            // record strikezone position
            if (self.translation.y - self.strikezone_panels_y.0).abs() < 0.5 {
                if (self.pos_at_strikezone_panels_y.0.y - self.strikezone_panels_y.0).abs()
                    > (self.translation.y - self.strikezone_panels_y.0).abs()
                {
                    self.pos_at_strikezone_panels_y.0 = self.translation;
                }
            }
            if (self.translation.y - self.strikezone_panels_y.1).abs() < 0.5 {
                if (self.pos_at_strikezone_panels_y.1.y - self.strikezone_panels_y.1).abs()
                    > (self.translation.y - self.strikezone_panels_y.1).abs()
                {
                    self.pos_at_strikezone_panels_y.1 = self.translation;
                }
            }
        }
    }

    // find seam indices that affect ssw
    // note that the local x-axis of the seams is the rotational axis
    // we need to calculate the velocity vector in relation to the seams' local bases
    fn find_ssw_seams(&self, ssw: &SeamShiftedWake) -> Vec<usize> {
        // let v_adjusted = self.in_seam_space(self.v);
        let rot_v = DQuat::from_rotation_arc(-DVec3::Y, self.v.normalize());
        let rot_spin = DQuat::from_rotation_x(ssw.seam_shift_factor * T_STEP);
        let (max, min) = ssw.get_activation_region();

        (0..N_SEAMS)
            .filter(|&i| {
                let point_adjusted = rot_v.mul_vec3(
                    rot_v
                        .inverse()
                        .mul_vec3(rot_spin.inverse().mul_vec3(self.seams[i])),
                );
                if (point_adjusted.x < max.x)
                    && (point_adjusted.x > min.x)
                    // within y range
                    && (point_adjusted.y < max.y)
                    && (point_adjusted.y > min.y)
                    // within z range
                    && (point_adjusted.z < max.z)
                    && (point_adjusted.z > min.z)
                {
                    self.outside_separated_flow(ssw, i)
                } else {
                    false
                }
            })
            .collect::<Vec<_>>()
    }

    /// since seams in the activation region cannot cause a separated flow to
    /// become separated again this function will eliminate any inline seams
    fn outside_separated_flow(&self, ssw: &SeamShiftedWake, index: usize) -> bool {
        let point = &self.seams[index];
        let next_point = &self.seams[(index + 1) % N_SEAMS];
        let prev_point = &self.seams[(index + N_SEAMS - 1) % N_SEAMS];
        let normalized_v: &DVec3 = &self.v.normalize();

        let angle_d = normalized_v.dot((*point - *prev_point).normalize()).acos();
        let angle_u = normalized_v.dot((*next_point - *point).normalize()).acos();

        (angle_d - PI_64).abs() >= ssw.separated_flow_range
            && (angle_u - PI_64).abs() >= ssw.separated_flow_range
    }

    fn rk4(&self, config: &BaseballPluginConfig, active_seams: &Vec<usize>) -> DVec3 {
        let spin = &self.spin;
        let seams = &self.seams;
        let time_elapsed = self.time_elapsed as f64;

        let v_1 = self.v;
        let t_1 = time_elapsed;
        let a_1 = Self::derivs(config, &v_1, spin, seams, t_1, active_seams);

        let v_2 = v_1 + a_1 * T_STEP * 0.5;
        let t_2 = t_1 + T_STEP * 0.5;
        let a_2 = Self::derivs(config, &v_2, spin, seams, t_2, active_seams);

        let v_3 = v_2 + a_2 * T_STEP * 0.5;
        let t_3 = t_2 + T_STEP * 0.5;
        let a_3 = Self::derivs(config, &v_3, spin, seams, t_3, active_seams);

        let v_4 = v_3 + a_3 * T_STEP;
        let t_4 = t_3 + T_STEP;
        let a_4 = Self::derivs(config, &v_4, spin, seams, t_4, active_seams);

        let slope = (a_1 + 2. * (a_2 + a_3) + a_4) / 6.0;

        // if self.time_elapsed > 0. && self.time_elapsed < 0.012 {
        //     info!("a_1 {:?}", a_1);
        //     info!("a_2 {:?}", a_2);
        //     info!("a_3 {:?}", a_3);
        //     info!("a_4 {:?}", a_4);
        // }

        slope
    }

    fn derivs(
        config: &BaseballPluginConfig,
        v: &DVec3,
        spin: &DVec3,
        seams: &Vec<DVec3>,
        time_elapsed: f64,
        active_seams: &Vec<usize>,
    ) -> DVec3 {
        let v_tot = v.length();
        let spin_rate = spin.length();

        let rw = (DIAMETER / 2.) * spin_rate;
        let s = (rw / v_tot) * (-time_elapsed / SPIN_DECAY).exp();
        let cl = 1. / (2.42 + (0.4 / s));

        // drag force
        let a_drag = if config.drag_on {
            *v * -C_0 * CD_CONST * v_tot
        } else {
            DVec3::ZERO
        };

        // magnus force
        let a_spin = if config.magnus_on {
            let [u, v, w] = v.to_array();
            let [spin_x, spin_y, spin_z] = spin.to_array();
            DVec3::new(
                spin_y * w - spin_z * v,
                spin_z * u - spin_x * w,
                spin_x * v - spin_y * u,
            ) * C_0
                * (cl / spin_rate)
                * v_tot
        } else {
            DVec3::ZERO
        };

        // ssw
        let a_ssw = if config.ssw_on {
            let seams_length = active_seams
                .iter()
                .fold(DVec3::ZERO, |s_length, &i| s_length + seams[i]);
            seams_length * -C_0 * C_SEAMS * v_tot.powi(2)
        } else {
            DVec3::ZERO
        };

        a_drag + a_spin + a_ssw
    }
}
