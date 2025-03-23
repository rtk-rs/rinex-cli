use hifitime::{Epoch, Unit};

pub trait CenteredDataPoints<T> {
    fn zero() -> T;
}

pub struct CenteredSnapshot<const M: usize, T: Copy + CenteredDataPoints<T>> {
    size: usize,
    inner: [(Epoch, T); M],
}

impl<const M: usize, T: Copy + CenteredDataPoints<T>> CenteredSnapshot<M, T> {
    pub fn new() -> Self {
        Self {
            size: 0,
            inner: [(Epoch::default(), T::zero()); M],
        }
    }

    pub fn rotate(&mut self) {
        self.size -= 1;
        self.inner.rotate_right(1);
    }

    pub fn valid(&self) -> bool {
        self.size == M
    }

    pub fn insert(&mut self, x: Epoch, y: T) {
        self.rotate();
        self.inner[0] = (x, y);
        if self.size < M {
            self.size += 1;
        }
    }

    pub fn centered(&self, x: Epoch) -> bool {
        if !self.valid() {
            return false;
        }
        let x_i = self.inner[M / 2].0;
        let dt_i = (x - x_i).abs();
        dt_i < 2.0 * Unit::Nanosecond
    }

    pub fn interpolate<F: Fn(&[(Epoch, T); M]) -> T>(&self, interp: F) -> T {
        interp(&self.inner)
    }
}
