use gnss_rtk::prelude::{Constellation, Duration, Time as RTKTime, TimeScale};

pub struct Time {}

impl RTKTime for Time {
    fn gst_gpst_offset_update(&mut self) -> Option<Duration> {
        None
    }

    fn bdt_gpst_offset_update(&mut self) -> Option<Duration> {
        None
    }

    fn bdt_gst_offset_update(&mut self) -> Option<Duration> {
        None
    }
}

impl Time {
    pub fn new() -> Self {
        Self {}
    }
}
