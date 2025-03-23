use std::collections::HashMap;

use rinex::{
    navigation::Ephemeris,
    prelude::{Epoch, SV},
};

use crate::{cli::Context, positioning::Buffer};

pub struct EphemerisSource<'a> {
    eos: bool,
    sv_buffers: HashMap<SV, Buffer<(Epoch, Ephemeris)>>,
    iter: Box<dyn Iterator<Item = (SV, Epoch, Epoch, &'a Ephemeris)> + 'a>,
}

impl<'a> EphemerisSource<'a> {
    /// Builds new [EphemerisSource] from [Context]
    pub fn from_ctx(ctx: &'a Context) -> Self {
        let brdc = ctx
            .data
            .brdc_navigation()
            .expect("Navigation RINEX is currently mandatory..");

        info!("Ephemeris data source created.");

        Self {
            eos: false,
            sv_buffers: HashMap::with_capacity(32),
            iter: Box::new(brdc.nav_ephemeris_frames_iter().filter_map(|(k, v)| {
                let sv_ts = k.sv.timescale()?;
                let toe = v.toe(sv_ts)?;
                Some((k.sv, k.epoch, toe, v))
            })),
        }
    }

    /// Consume one entry from [Iterator]
    fn consume_one(&mut self) {
        if let Some((sv, toc, toe, eph)) = self.iter.next() {
            if let Some(buffer) = self.sv_buffers.get_mut(&sv) {
                buffer.push(toc, (toe, eph.clone()));
            } else {
                let mut buffer = Buffer::new(4);
                buffer.push(toc, (toe, eph.clone()));
                self.sv_buffers.insert(sv, buffer);
            }
        } else {
            if !self.eos {
                info!("Consumed all Ephemeris.");
            }

            self.eos = true;
        }
    }

    /// [Ephemeris] selection attempt, for [SV] at [Epoch]
    fn try_select(&self, t: Epoch, sv: SV) -> Option<(Epoch, Epoch, &Ephemeris)> {
        let buffer = self.sv_buffers.get(&sv)?;

        if sv.constellation.is_sbas() {
            buffer
                .inner
                .iter()
                .filter_map(|(toc_i, (toe_i, eph_i))| {
                    if t >= *toc_i {
                        Some((*toc_i, *toe_i, eph_i))
                    } else {
                        None
                    }
                })
                .min_by_key(|(toc_i, _, _)| (t - *toc_i).abs())
        } else {
            buffer
                .inner
                .iter()
                .filter_map(|(toc_i, (toe_i, eph_i))| {
                    if eph_i.is_valid(sv, t, *toe_i) {
                        Some((*toc_i, *toe_i, eph_i))
                    } else {
                        None
                    }
                })
                .min_by_key(|(_, toe_i, _)| (t - *toe_i).abs())
        }
    }

    /// [Ephemeris] selection at [Epoch] for [SV].
    pub fn select(&mut self, t: Epoch, sv: SV) -> Option<(Epoch, Epoch, Ephemeris)> {
        while !self.eos {
            if let Some((toc_i, toe_i, eph_i)) = self.try_select(t, sv) {
                return Some((toc_i, toe_i, eph_i.clone()));
            } else {
                self.consume_one();
                if self.eos {
                    return None;
                }
            }
        }

        None
    }
}
