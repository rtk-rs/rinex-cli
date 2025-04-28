use crate::cli::Context;
use gnss_qc::prelude::GnssAbsoluteTime;
use gnss_rtk::prelude::{AbsoluteTime, Epoch, TimeScale};
use hifitime::Unit;

pub struct Time {
    solver: GnssAbsoluteTime,
}

impl AbsoluteTime for Time {
    fn new_epoch(&mut self, now: Epoch) {
        self.solver.outdate_weekly(now);
    }

    fn epoch_correction(&self, t: Epoch, target: TimeScale) -> Epoch {
        // try to run the precise correction, if we have such information in the database
        if let Some(corrected) = self.solver.precise_epoch_correction(t, target) {
            corrected
        } else {
            // otherwise, rely on coarse conversion
            t.to_time_scale(target)
        }
    }
}

impl Time {
    pub fn new(ctx: &Context) -> Self {
        Self {
            solver: ctx.data.gnss_absolute_time_solver(),
        }
    }
}
