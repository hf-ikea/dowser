pub mod util {
    use std::f64::consts::PI;

    use num_complex::{Complex, ComplexFloat};

    /// in db
    pub fn reflection_loss(z_load: Complex<f64>, z_source: Complex<f64>) -> f64 {
        -20.0 * f64::log10(((z_load - z_source) / (z_load + z_source)).abs())
    }

    pub fn get_refl_coef(z_load: Complex<f64>, z_source: Complex<f64>) -> Complex<f64> {
        (z_load - z_source) / (z_load + z_source)
    }

    pub fn swr(z_load: Complex<f64>, z_source: Complex<f64>) -> f64 {
        let refl_abs: f64 = get_refl_coef(z_load, z_source).abs();
        (1.0 + refl_abs) / (1.0 - refl_abs)
    }

    pub fn hz_to_angular_freq(f: f64) -> f64 {
        2.0 * PI * f
    }
}