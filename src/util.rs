pub mod util {
    use std::f64::consts::PI;

    use num_complex::{Complex, ComplexFloat};

    const FREE_SPACE_PERMEABILITY: f64 = 12.5663706144e-7;

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
        503.0 * (resistivity / (permeability * f)).sqrt()
    }

    /// in ohms
    pub fn get_rf_resistance(skin_depth: f64, diameter: f64, length: f64, resistivity: f64) -> f64{
        (length * resistivity) / (PI * skin_depth * (diameter - skin_depth))
    }

    pub fn get_coax_inductance(rel_permeability: f64, inner_conductor_diameter: f64, inner_shield_diameter: f64, length: f64) -> f64 {
        ((rel_permeability * FREE_SPACE_PERMEABILITY * length) / (2.0 * PI)) * ((inner_conductor_diameter / inner_shield_diameter).ln())
    }
}

#[cfg(test)]
mod tests {
    use crate::util::util::{get_coax_inductance, get_rf_resistance, get_skin_depth};

    #[test]
    fn test_rf_resistance() {
        let resistivity: f64 = 2.44e-8; // gold
        let permeability: f64 = 1.0;
        let f: f64 = 1000.0;
        let skin_depth: f64 = get_skin_depth(f, permeability, resistivity);
        dbg!(skin_depth);
        let rf_resistance: f64 = get_rf_resistance(skin_depth, 0.033, 1.0, resistivity);
        dbg!(rf_resistance);
    }

    #[test]
    fn test_coax_inductance() {
        let permeability: f64 = 1.0;
        let inner_shield_diameter: f64 = 0.030;
        let inner_conductor_diameter: f64 = 0.010;
        let length: f64 = 0.001;
        dbg!(get_coax_inductance(permeability, inner_conductor_diameter, inner_shield_diameter, length));
    }
}