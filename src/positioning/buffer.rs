use hifitime::Epoch;

use crate::positioning::{CenteredDataPoints, CenteredSnapshot};

pub struct Buffer<T> {
    pub last_t: Epoch,
    inner: Vec<(Epoch, T)>,
}

impl<T: Copy + CenteredDataPoints<T>> Buffer<T> {
    pub fn new(malloc: usize) -> Self {
        Self {
            last_t: Epoch::default(),
            inner: Vec::with_capacity(malloc),
        }
    }

    // pub fn contains(&self, x: &Epoch) -> Option<&T> {
    //     self.inner
    //         .iter()
    //         .filter(|(x_i, _)| x_i == x)
    //         .reduce(|k, _| k)
    //         .map(|(_, y)| y)
    // }

    pub fn push(&mut self, x: Epoch, y: T) {
        self.inner.push((x, y));
        self.last_t = x;
    }

    /// Obtained centered [CenteredSnapshot] around desired [Epoch].
    pub fn centered_snapshot<const M: usize>(
        &self,
        t: Epoch,
        snapshot: &mut CenteredSnapshot<M, T>,
    ) {
        for (x_i, y_i) in self.inner.iter() {
            snapshot.insert(*x_i, *y_i);

            if snapshot.valid() {
                if snapshot.centered(t) {
                    return;
                }
            }
        }
    }
}
