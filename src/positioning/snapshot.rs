use hifitime::{Duration, Epoch};

pub trait CenteredDataPoints<T> {
    fn zero() -> T;
}

pub struct CenteredSnapshot<const M: usize, T: Copy + CenteredDataPoints<T>> {
    pub size: usize,
    inner: [(Epoch, T); M],
}

impl<const M: usize, T: Copy + CenteredDataPoints<T>> CenteredSnapshot<M, T> {
    pub fn new() -> Self {
        assert!(M % 2 == 0, "only odd interpolation orders supported");
        Self {
            size: 0,
            inner: [(Epoch::default(), T::zero()); M],
        }
    }

    pub fn rotate(&mut self) {
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

    pub fn centered(&self, x: Epoch, dt: Duration) -> bool {
        if !self.valid() {
            return false;
        }
        let x_i = self.inner[M / 2 - 1].0;
        debug!("x_i={} | target={}", x_i, x);

        let dt_i = (x - x_i).abs();
        dt_i < dt
    }

    pub fn interpolate<F: Fn(&[(Epoch, T); M]) -> T>(&self, interp: F) -> T {
        interp(&self.inner)
    }
}
