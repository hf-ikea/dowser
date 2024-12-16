use core::f64;
use num_complex::Complex;
use spec_math::cephes64::sici;
use std::f64::consts::PI;

use crate::{antenna::{AntennaModel, AntennaProperties, ModeledAntenna}, consts::{FREE_SPACE_IMPEDANCE, GAMMA, SPEED_OF_LIGHT}, util::sin2};

pub struct DipoleProperties {
    /// Dipole length in meters
    pub length: f64,
    /// Dipole diameter in meters
    pub diameter: f64,
}

impl AntennaModel for DipoleProperties {
    fn model(&self, properties: AntennaProperties) -> ModeledAntenna {
        ModeledAntenna::new(properties, z(properties.frequency, self.length, self.diameter))
    }
}

pub fn r(f: f64, l: f64) -> f64 {
    let k = 2.0 * PI * f / SPEED_OF_LIGHT;

    FREE_SPACE_IMPEDANCE / (2.0 * PI * sin2(k * l / 2.0))
        * (GAMMA + f64::ln(k * l) - sici(k * l).1
            + (0.5 * f64::sin(k * l) * (sici(2.0 * k * l).0 - (2.0 * sici(k * l).0)))
            + (0.5
                * f64::cos(k * l)
                * (GAMMA + f64::ln(k * l / 2.0) + sici(2.0 * k * l).1 - (2.0 * sici(k * l).1))))
}

pub fn x(f: f64, l: f64, a: f64) -> f64 {
    let k = 2.0 * PI * f / SPEED_OF_LIGHT;

    FREE_SPACE_IMPEDANCE / (4.0 * PI * sin2(k * l / 2.0))
        * (2.0 * sici(k * l).0 + (f64::cos(k * l) * (2.0 * sici(k * l).0 - sici(2.0 * k * l).0))
            - (f64::sin(k * l)
                * (2.0 * sici(k * l).1 - sici(2.0 * k * l).1 - sici(2.0 * k * a.powf(2.0) / l).1)))
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

#[cfg(test)]
mod tests {
    use num_complex::Complex;

    use super::{z, DipoleProperties};

    use crate::{antenna::{AntennaPolarization, AntennaProperties}, util::swr};

    #[test]
    fn test_dipole_swr_sim() {
        let f_lower: f64 = 14.1e6;
        let f_upper: f64 = 15e6;
        let step: f64 = 30e3;

        let length: f64 = 10.0; // meters
        let diameter: f64 = 2.053e-3; // meters

        let source: Complex<f64> = Complex::new(50.0, 0.0); // 50 ohms

        let properties: AntennaProperties = AntennaProperties {
            frequency: 2000e6,
            orientation: 0.0,
            polarization: AntennaPolarization::Horizontal,
            z_s: Complex::new(50.0, 0.0),
        };
        let coax: DipoleProperties = DipoleProperties {
            length: 10.0,
            diameter: 2.053e-3,
        };

        let mut f: f64 = f_lower;
        while f < f_upper {
            let load: Complex<f64> = z(f, length, diameter);
            println!(
                "{} MHz: SWR {}, {} + j{}",
                f / 1e6,
                swr(load, source),
                load.re,
                load.im
            );
            f += step;
        }
    }
}