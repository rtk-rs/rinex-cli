use hifitime::{Duration, Epoch};

use crate::positioning::{CenteredDataPoints, CenteredSnapshot};

pub struct Buffer<T> {
    pub last_t: Epoch,
    pub inner: Vec<(Epoch, T)>,
}

impl<T> Buffer<T> {
    pub fn new(malloc: usize) -> Self {
        Self {
            last_t: Epoch::default(),
            inner: Vec::with_capacity(malloc),
        }
    }

    pub fn push(&mut self, x: Epoch, y: T) {
        self.inner.push((x, y));
        self.last_t = x;
    }
}

impl<T: Copy + CenteredDataPoints<T>> Buffer<T> {
    /// Obtained centered [CenteredSnapshot] around desired [Epoch].
    pub fn centered_snapshot<const M: usize>(
        &self,
        t: Epoch,
        dt: Duration,
        snapshot: &mut CenteredSnapshot<M, T>,
    ) {
        snapshot.size = 0; // reset

        // TODO: improve overall implementation
        // and start from previous pointer position
        for (x_i, y_i) in self.inner.iter() {
            snapshot.insert(*x_i, *y_i);

            if snapshot.valid() {
                if snapshot.centered(t, dt) {
                    return;
                }
            }
        }
    }
}
