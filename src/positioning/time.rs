use gnss_rtk::prelude::{Epoch, Time as RTKTime, TimeOffset as RTKTimeOffset, TimeScale};

use rinex::navigation::TimeOffset as RINEXTimeOffset;

use crate::cli::Context;

struct HeaderTimeOffset {
    time_offset: RINEXTimeOffset,
    published: bool,
}

pub struct Time {
    header_offsets: Vec<HeaderTimeOffset>,
}

impl RTKTime for Time {
    fn bdt_gpst_time_offset(&mut self, _: Epoch) -> Option<RTKTimeOffset> {
        if let Some(header_offset) = self.header_offsets.iter_mut().find(|hd| {
            !hd.published
                && hd.time_offset.lhs == TimeScale::BDT
                && hd.time_offset.rhs == TimeScale::GPST
        }) {
            header_offset.published = true;

            Some(RTKTimeOffset::from_bdt_gpst_time_of_week(
                header_offset.time_offset.t_ref,
                header_offset.time_offset.polynomial,
            ))
        } else {
            // TODO
            None
        }
    }

    fn bdt_gst_time_offset(&mut self, _: Epoch) -> Option<RTKTimeOffset> {
        if let Some(header_offset) = self.header_offsets.iter_mut().find(|hd| {
            !hd.published
                && hd.time_offset.lhs == TimeScale::BDT
                && hd.time_offset.rhs == TimeScale::GST
        }) {
            header_offset.published = true;

            Some(RTKTimeOffset::from_bdt_gst_time_of_week(
                header_offset.time_offset.t_ref,
                header_offset.time_offset.polynomial,
            ))
        } else {
            // TODO
            None
        }
    }

    fn bdt_utc_time_offset(&mut self, _: Epoch) -> Option<RTKTimeOffset> {
        if let Some(header_offset) = self.header_offsets.iter_mut().find(|hd| {
            !hd.published
                && hd.time_offset.lhs == TimeScale::BDT
                && hd.time_offset.rhs == TimeScale::UTC
        }) {
            header_offset.published = true;

            Some(RTKTimeOffset::from_bdt_utc_time_of_week(
                header_offset.time_offset.t_ref,
                header_offset.time_offset.polynomial,
            ))
        } else {
            // TODO
            None
        }
    }

    fn gpst_utc_time_offset(&mut self, _: Epoch) -> Option<RTKTimeOffset> {
        if let Some(header_offset) = self.header_offsets.iter_mut().find(|hd| {
            !hd.published
                && hd.time_offset.lhs == TimeScale::GPST
                && hd.time_offset.rhs == TimeScale::UTC
        }) {
            header_offset.published = true;

            Some(RTKTimeOffset::from_gpst_utc_time_of_week(
                header_offset.time_offset.t_ref,
                header_offset.time_offset.polynomial,
            ))
        } else {
            // TODO
            None
        }
    }

    fn gst_gpst_time_offset(&mut self, _: Epoch) -> Option<RTKTimeOffset> {
        if let Some(header_offset) = self.header_offsets.iter_mut().find(|hd| {
            !hd.published
                && hd.time_offset.lhs == TimeScale::GST
                && hd.time_offset.rhs == TimeScale::GPST
        }) {
            header_offset.published = true;

            Some(RTKTimeOffset::from_gst_gpst_time_of_week(
                header_offset.time_offset.t_ref,
                header_offset.time_offset.polynomial,
            ))
        } else {
            // TODO
            None
        }
    }

    fn gst_utc_time_offset(&mut self, _: Epoch) -> Option<RTKTimeOffset> {
        if let Some(header_offset) = self.header_offsets.iter_mut().find(|hd| {
            !hd.published
                && hd.time_offset.lhs == TimeScale::GST
                && hd.time_offset.rhs == TimeScale::UTC
        }) {
            header_offset.published = true;

            Some(RTKTimeOffset::from_gst_utc_time_of_week(
                header_offset.time_offset.t_ref,
                header_offset.time_offset.polynomial,
            ))
        } else {
            // TODO
            None
        }
    }
}

impl Time {
    pub fn new(ctx: &Context) -> Self {
        Self {
            header_offsets: if let Some(brdc) = ctx.data.brdc_navigation() {
                let header = brdc
                    .header
                    .nav
                    .as_ref()
                    .expect("invalid NAV rinex: empty header");

                let offsets = &header.time_offsets;

                offsets
                    .iter()
                    .map(|offset| HeaderTimeOffset {
                        time_offset: offset.clone(),
                        published: false,
                    })
                    .collect::<Vec<_>>()
            } else {
                Default::default()
            },
        }
    }
}
