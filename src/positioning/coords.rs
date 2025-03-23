use crate::positioning::CenteredDataPoints;

#[derive(Debug, Default, Clone, Copy)]
pub struct Coords3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Coords3d {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl CenteredDataPoints<Coords3d> for Coords3d {
    fn zero() -> Coords3d {
        Coords3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}
