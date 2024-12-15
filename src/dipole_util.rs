mod dipole_util {
    use core::f64;
    use std::f64::consts::PI;
    use spec_math::cephes64::sici;
    use num_complex::Complex;

    const GAMMA: f64 = 0.577215664901532860606512;
    const FREE_SPACE_IMPEDANCE: f64 = 376.730313;
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

    pub fn z(f: f64, length: f64, diameter: f64) -> Complex<f64> {
        Complex::new(r(f, length), x(f, length, diameter))
    }

    /// What I like to call the "good enough" model, ignores the ground and differences in non-resonant antennas
    /// theta is in radians of course, this outputs dbi
    /// ""directivity""
    pub fn gain(theta: f64) -> f64 {
        sin2(theta) * 3.15
    }

    
}

#[cfg(test)]
mod tests {
    use num_complex::Complex;

    use super::dipole_util::z;

    use crate::util::util::swr;

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