use crate::consts::SPEED_OF_LIGHT;
use std::f64::consts::PI;

/// this comes with all the stuff you would assume free space means
pub fn fspl(d: f64, f: f64) -> f64 {
    (4.0 * PI * d * SPEED_OF_LIGHT) / f
}
