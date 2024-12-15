mod propagation {
    use std::f64::consts::PI;
    const SPEED_OF_LIGHT: f64 = 299792458.0;

    /// this comes with all the stuff you would assume free space means
    pub fn fspl(d: f64, f: f64) -> f64 {
        (4.0 * PI * d * SPEED_OF_LIGHT) / f
    }
}