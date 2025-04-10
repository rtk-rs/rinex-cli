use gnss_rtk::prelude::{Constellation, Duration, Epoch, Time as RTKTime, TimeScale};

use rinex::navigation::TimeOffset;

use crate::cli::Context;

pub struct Time {
    header_forwarded: bool,
    time_offsets: Vec<TimeOffset>,
}

impl RTKTime for Time {
    fn gpst_utc_offset_update(&mut self, t: Epoch) -> Option<Duration> {
        let (lhs, rhs) = (TimeScale::GPST, TimeScale::UTC);

        if !self.header_forwarded {
            let time_offset = self
                .time_offsets
                .iter()
                .find(|k| k.lhs == lhs && k.rhs == rhs)?;
            let dt = time_offset.time_offset(t);
            Some(dt)
        } else {
            // TODO
            None
        }
    }

    fn gst_gpst_offset_update(&mut self, t: Epoch) -> Option<Duration> {
        let (lhs, rhs) = (TimeScale::GST, TimeScale::GPST);

        if !self.header_forwarded {
            let time_offset = self
                .time_offsets
                .iter()
                .find(|k| k.lhs == lhs && k.rhs == rhs)?;
            let dt = time_offset.time_offset(t);
            Some(dt)
        } else {
            // TODO
            None
        }
    }

    fn gst_utc_offset_update(&mut self, t: Epoch) -> Option<Duration> {
        let (lhs, rhs) = (TimeScale::GST, TimeScale::UTC);

        if !self.header_forwarded {
            let time_offset = self
                .time_offsets
                .iter()
                .find(|k| k.lhs == lhs && k.rhs == rhs)?;
            let dt = time_offset.time_offset(t);
            Some(dt)
        } else {
            // TODO
            None
        }
    }

    fn bdt_gpst_offset_update(&mut self, t: Epoch) -> Option<Duration> {
        let (lhs, rhs) = (TimeScale::BDT, TimeScale::GPST);

        if !self.header_forwarded {
            let time_offset = self
                .time_offsets
                .iter()
                .find(|k| k.lhs == lhs && k.rhs == rhs)?;
            let dt = time_offset.time_offset(t);
            Some(dt)
        } else {
            // TODO
            None
        }
    }

    fn bdt_utc_offset_update(&mut self, t: Epoch) -> Option<Duration> {
        None
    }

    fn bdt_gst_offset_update(&mut self, t: Epoch) -> Option<Duration> {
        None
    }
}

impl Time {
    pub fn new(ctx: &Context) -> Self {
        Self {
            header_forwarded: false,
            time_offsets: if let Some(brdc) = ctx.data.brdc_navigation() {
                let header = brdc
                    .header
                    .nav
                    .as_ref()
                    .expect("invalid NAV rinex: empty header");

                header.time_offsets.clone()
            } else {
                Default::default()
            },
        }
    }
}
