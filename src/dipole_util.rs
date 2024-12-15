mod dipole_util {
    use core::f64;
    use std::f64::consts::PI;
    use spec_math::cephes64::sici;
    use num_complex::{Complex, ComplexFloat};

    const GAMMA: f64 = 0.577215664901532860606512;
    const FREE_SPACE_IMPEDANCE: f64 = 376.73031;
    const SPEED_OF_LIGHT: f64 = 299792458.0;
    pub fn sin2(x: f64) -> f64 {
        f64::sin(x).powf(2.0)
    }

    pub fn r(f: f64, l: f64) -> f64 {
        let k = 2.0 * PI * f / SPEED_OF_LIGHT;

        return FREE_SPACE_IMPEDANCE / (2.0 * PI * sin2(k * l / 2.0)) * (
            GAMMA
            + f64::ln(k * l)
            - sici(k * l).1
            + (0.5 * f64::sin(k * l) * (
                sici(2.0 * k * l).0
                - (2.0 * sici(k * l).0)
            ))
            + (0.5 * f64::cos(k * l) * (
                GAMMA
                + f64::ln(k * l / 2.0)
                + sici(2.0 * k * l).1
                - (2.0 * sici(k * l).1)
            ))
        )
    }

    pub fn x(f: f64, l: f64, a: f64) -> f64 {
        let k = 2.0 * PI * f / SPEED_OF_LIGHT;

        return FREE_SPACE_IMPEDANCE / (4.0 * PI * sin2(k * l / 2.0)) * (
            2.0 * sici(k * l).0
            + (f64::cos(k * l) * (
                2.0 * sici(k * l).0
                - sici(2.0 * k * l).0
            ))
            - (f64::sin(k * l) * (
                2.0 * sici(k * l).1
                - sici(2.0 * k * l).1
                - sici(2.0 * k * a.powf(2.0) / l).1
            ))
        )
    }

    pub fn z(f: f64, l: f64, a: f64) -> Complex<f64> {
        Complex::new(r(f, l), x(f, l, a))
    }

    pub fn antenna_loss(z_load: f64, z_source: f64) -> f64 {
        //in dB
        -20.0 * f64::log10(((z_load - z_source) / (z_load + z_source)).abs())
    }

    pub fn swr(z_load: Complex<f64>, z_source: Complex<f64>) -> f64 {
        let refl_coef: Complex<f64> = (z_load - z_source) / (z_load + z_source);
        let refl_abs: f64 = refl_coef.abs();
        (1.0 + refl_abs) / (1.0 - refl_abs)
    }
}

#[cfg(test)]
mod tests {
    use num_complex::Complex;

    use crate::dipole_util::dipole_util::swr;

    use super::dipole_util::z;

    #[test]
    fn test_dipole_swr_sim() {
        let f_lower: f64 = 14.1e6;
        let f_upper: f64 = 15e6;
        let step: f64 = 30e3;

        let length: f64 = 10.0; // meters
        let diameter: f64 = 2.053e-3; // meters

        let source: Complex<f64> = Complex::new(50.0, 0.0); // 50 ohms

        let mut f: f64 = f_lower;
        while f < f_upper {
            let load: Complex<f64> = z(f, length, diameter);
            println!("{} MHz: SWR {}, {} + j{}", f / 1e6, swr(load, source), load.re, load.im);
            f += step;
        }
    }
}