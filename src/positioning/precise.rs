use gnss_rtk::prelude::{
    Almanac, Duration, Epoch, Frame, Orbit, Vector3, EARTH_J2000, SUN_J2000, SV,
};

use std::collections::HashMap;

use crate::{
    cli::Context,
    positioning::{Buffer, CenteredSnapshot, Coords3d},
};

use rinex::prelude::Carrier;

use anise::errors::AlmanacError;

const INTERP_ORDER: usize = 11;
const BUFFER_SIZE: usize = INTERP_ORDER + 1;
const INTERP_ORDER_X2: usize = INTERP_ORDER * 2;

pub struct PreciseOrbits<'a> {
    /// End of stream
    eos: bool,

    /// Sampling period
    sampling_period: Duration,

    /// SV buffers
    sv_buffers: HashMap<SV, Buffer<Coords3d>>,

    /// Iterator
    iter: Box<dyn Iterator<Item = (Epoch, SV, (f64, f64, f64))> + 'a>,

    /// SV [CenteredSnapshot]
    sv_snapshots: HashMap<SV, CenteredSnapshot<BUFFER_SIZE, Coords3d>>,
}

fn sun_unit_vector(almanac: &Almanac, t: Epoch) -> Result<Vector3<f64>, AlmanacError> {
    let earth_sun = almanac.transform(EARTH_J2000, SUN_J2000, t, None)?;
    Ok(Vector3::new(
        earth_sun.radius_km.x * 1000.0,
        earth_sun.radius_km.y * 1000.0,
        earth_sun.radius_km.z * 1000.0,
    ))
}

impl<'a> PreciseOrbits<'a> {
    pub fn new(ctx: &'a Context) -> Self {
        Self {
            eos: false,
            sv_buffers: HashMap::with_capacity(16),
            sv_snapshots: HashMap::with_capacity(16),
            sampling_period: if let Some(sp3) = ctx.data.sp3() {
                sp3.header.sampling_period
            } else {
                Duration::default()
            },
            iter: {
                if let Some(sp3) = ctx.data.sp3() {
                    if let Some(atx) = ctx.data.antex() {
                        info!("Orbit source created: operating with Ultra Precise Orbits.");
                        Box::new(sp3.satellites_position_km_iter().filter_map(
                            |(t, sv, _predicted, _maneuvered, (x_km, y_km, z_km))| {
                                // TODO: needs rework and support all frequencies
                                let delta = atx.sv_antenna_apc_offset(t, sv, Carrier::L1)?;
                                let delta = Vector3::new(delta.0, delta.1, delta.2);
                                let r_sat = Vector3::new(x_km * 1.0E3, y_km * 1.0E3, z_km * 1.0E3);
                                let k = -r_sat
                                    / (r_sat[0].powi(2) + r_sat[1].powi(2) + r_sat[3].powi(2))
                                        .sqrt();

                                let r_sun = sun_unit_vector(&ctx.data.almanac, t).ok()?;

                                let norm = ((r_sun[0] - r_sat[0]).powi(2)
                                    + (r_sun[1] - r_sat[1]).powi(2)
                                    + (r_sun[2] - r_sat[2]).powi(2))
                                .sqrt();

                                let e = (r_sun - r_sat) / norm;
                                let j = Vector3::new(k[0] * e[0], k[1] * e[1], k[2] * e[2]);
                                let i = Vector3::new(j[0] * k[0], j[1] * k[1], j[2] * k[2]);

                                let r_dot = Vector3::new(
                                    (i[0] + j[0] + k[0]) * delta[0],
                                    (i[1] + j[1] + k[1]) * delta[1],
                                    (i[2] + j[2] + k[2]) * delta[2],
                                );

                                let r_sat = r_sat - r_dot;

                                Some((t, sv, (r_sat[0], r_sat[1], r_sat[2])))
                            },
                        ))
                    } else {
                        info!("Orbit source created: operating with Precise Orbits.");
                        warn!("Cannot determine exact precise coordinates without ANTEX database");
                        warn!("Expect tiny errors in your results (<1m).");
                        Box::new(sp3.satellites_position_km_iter().map(
                            |(t, sv, _predicted, _maneuvered, coordinates)| (t, sv, coordinates),
                        ))
                    }
                } else {
                    info!("Orbit source created: operating without Precise Orbits");
                    Box::new([].into_iter())
                }
            },
        }
    }

    /// Consume one symbol from iterator
    fn consume_one(&mut self) {
        if let Some((t, sv, (x_km, y_km, z_km))) = self.iter.next() {
            let coords = Coords3d::new(x_km, y_km, z_km);

            if let Some(buf) = self.sv_buffers.get_mut(&sv) {
                buf.push(t, coords);
            } else {
                let mut buf = Buffer::<Coords3d>::new(32);
                buf.push(t, coords);
                self.sv_buffers.insert(sv, buf);
            }
        } else {
            info!("Consumed all precise coordinates.");
            self.eos = true;
        }
    }

    fn consume_many(&mut self, n: usize) {
        let mut i = 0;
        while !self.eos && i < n {
            self.consume_one();
            i += 1;
        }
    }

    /// Attempts to return a precise state at desired [Epoch].
    pub fn next_precise_at(&mut self, t: Epoch, sv: SV, frame: Frame) -> Option<Orbit> {
        let min_t = t - ((INTERP_ORDER + 1) / 2) as f64 * self.sampling_period;
        let max_t = t + ((INTERP_ORDER + 1) / 2) as f64 * self.sampling_period;

        while !self.eos {
            self.consume_many(INTERP_ORDER_X2);

            if let Some(buffer) = self.sv_buffers.get(&sv) {
                if buffer.last_t >= max_t {
                    break;
                }
            }
        }

        // centered buffer
        if self.sv_snapshots.get(&sv).is_none() {
            self.sv_snapshots.insert(sv, CenteredSnapshot::new());
        }

        let sv_snapshot = self.sv_snapshots.get_mut(&sv).unwrap();
        let sv_buffer = self.sv_buffers.get(&sv)?;

        sv_buffer.centered_snapshot(t, self.sampling_period, sv_snapshot);

        if !sv_snapshot.centered(t, self.sampling_period) {
            return None;
        }

        let coords_km = sv_snapshot.interpolate(|buf| {
            let mut polynomials = (0.0_f64, 0.0_f64, 0.0_f64);

            for i in 0..=INTERP_ORDER {
                let mut li = 1.0_f64;
                let (t_i, coords_i) = buf[i];

                for j in 0..=INTERP_ORDER {
                    let (t_j, _) = buf[j];
                    if j != i {
                        li *= (t - t_j).to_seconds();
                        li /= (t_i - t_j).to_seconds();
                    }
                }

                polynomials.0 += coords_i.x * li;
                polynomials.1 += coords_i.y * li;
                polynomials.2 += coords_i.z * li;
            }

            Coords3d::new(polynomials.0, polynomials.1, polynomials.2)
        });

        Some(Orbit::from_position(
            coords_km.x,
            coords_km.y,
            coords_km.z,
            t,
            frame,
        ))
    }
}
