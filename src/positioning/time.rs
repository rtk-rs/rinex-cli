use crate::cli::Context;
use gnss_qc::prelude::TimeCorrectionsDB;
use hifitime::Unit;

use gnss_rtk::prelude::{AbsoluteTime, Epoch, TimeScale};

pub struct Time {
    database: Option<TimeCorrectionsDB>,
}

impl AbsoluteTime for Time {
    fn new_epoch(&mut self, now: Epoch) {
        if let Some(db) = &mut self.database {
            db.outdate_weekly(now);
        }
    }

    fn epoch_correction(&self, t: Epoch, timescale: TimeScale) -> Epoch {
        // try to apply a precise correction
        if let Some(db) = &self.database {
            match db.precise_epoch_correction(t, timescale) {
                Some(epoch) => epoch,
                None => {
                    // otherwise, rely on coarse conversion
                    t.to_time_scale(timescale)
                },
            }
        } else {
            // only coarse conversion possible
            t.to_time_scale(timescale)
        }
    }
}

impl Time {
    pub fn new(ctx: &Context) -> Self {
        Self {
            database: ctx.data.time_corrections_database(),
        }
    }
}
