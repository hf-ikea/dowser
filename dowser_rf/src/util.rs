use std::f64::consts::PI;

use num_complex::{Complex, ComplexFloat};

pub const FREE_SPACE_PERMITTIVITY: f64 = 8.85418781881e-12;
pub const FREE_SPACE_PERMEABILITY: f64 = 12.5663706144e-7;

pub fn coth(x: f64) -> f64 {
    x.cosh() / x.sinh()
}

pub fn coth_complex(x: Complex<f64>) -> Complex<f64> {
    x.cosh() / x.sinh()
}

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

pub fn get_skin_depth(f: f64, permeability: f64, resistivity: f64) -> f64 {
    (resistivity / (PI * f * permeability)).sqrt()
}

/// in ohm-meters
pub fn get_rf_resistance(skin_depth: f64, diameter: f64, resistivity: f64) -> f64 {
    (resistivity) / (PI * skin_depth * diameter)
}

#[cfg(test)]
mod tests {
    use crate::util::{get_rf_resistance, get_skin_depth, FREE_SPACE_PERMEABILITY};

    #[test]
    fn test_rf_resistance() {
        let resistivity: f64 = 2.44e-8; // gold
        let permeability: f64 = 1.0 * FREE_SPACE_PERMEABILITY;
        let f: f64 = 50.0;
        let skin_depth: f64 = get_skin_depth(f, permeability, resistivity);
        dbg!(skin_depth);
        let rf_resistance: f64 = get_rf_resistance(skin_depth, 0.033, resistivity);
        dbg!(rf_resistance);
    }
}
