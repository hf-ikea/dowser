mod coax_line {
    use std::f64::consts::PI;

    use num_complex::ComplexFloat;

    use crate::{trx_line::trx_line::TransmissionLineState, util::util::FREE_SPACE_PERMEABILITY};

    pub struct CoaxLineState {
        pub line_state: TransmissionLineState,
        pub inner_diameter: f64, // outer diameter of inter conductor in meters
        pub shield_diameter: f64, // inner diameter of shield in meters
        pub dielectric_constant: f64,
        pub magnetic_permeability: f64, // relative
    }

    impl CoaxLineState {
        pub fn setup_coax_state() -> CoaxLineState {
            let t_state: TransmissionLineState = TransmissionLineState::setup_line_state();
            CoaxLineState {
                line_state: t_state,
                inner_diameter: 0.0,
                shield_diameter: 0.0,
                dielectric_constant: 0.0,
                magnetic_permeability: 0.0
            }
        }

        pub fn set_capacitance(&mut self) {
            self.line_state.c = (2.0 * PI * self.dielectric_constant) / (self.shield_diameter / self.inner_diameter).ln();
        }

        pub fn set_inductance(&mut self) {
            self.line_state.l = Self::get_coax_inductance(self.magnetic_permeability, self.inner_diameter, self.shield_diameter);
        }

        pub fn set_resistance(&mut self, resistivity_inner: f64, resistivity_shield: f64) {
            self.line_state.r = (self.line_state.f * FREE_SPACE_PERMEABILITY / PI).sqrt() * ((resistivity_inner.sqrt() / self.inner_diameter) + (resistivity_shield.sqrt() / self.shield_diameter));
        }

        pub fn get_loss_per_meter(&mut self) -> f64 {
            // db/meter
            let resistive_loss: f64 = 4.34294 * self.line_state.r / self.line_state.z.abs();
            let dielectric_loss: f64 = 0.00409312451 * self.line_state.f * self.line_state.c * self.line_state.z.abs();
            dbg!(resistive_loss * self.line_state.length);
            dbg!(dielectric_loss * self.line_state.length);
            dielectric_loss + resistive_loss
            //8.68588 * self.line_state.r.sqrt() / (2.0 * self.line_state.z.abs())
        }

        // henry/meter
        pub fn get_coax_inductance(permeability: f64, inner_conductor_diameter: f64, inner_shield_diameter: f64) -> f64 {
            (permeability / (2.0 * PI)) * ((inner_shield_diameter / inner_conductor_diameter).ln())
        }
    }
}

#[cfg(test)]
mod tests {
    use num_complex::Complex;

    use crate::{trx_line::trx_line::TransmissionLineState, util::util::{FREE_SPACE_PERMEABILITY, FREE_SPACE_PERMITTIVITY}};

    use super::coax_line::*;

    #[test]
    fn test_coax_inductance() {
        let permeability: f64 = 1.0 * FREE_SPACE_PERMEABILITY;
        let inner_conductor_diameter: f64 = 0.00274;
        let inner_shield_diameter: f64 = 0.00739;
        let length: f64 = 1.0;
        dbg!(CoaxLineState::get_coax_inductance(permeability, inner_conductor_diameter, inner_shield_diameter) * length);
    }

    #[test]
    fn test_coax_line() {
        let line_state: TransmissionLineState = TransmissionLineState {
            f: 2500e6,
            length: 100.0,
            z: Complex::ZERO,
            r: 0.0,
            l: 0.0,
            c: 0.0,
            g: 0.0,
            gamma: Complex::ZERO,
        };
        let mut coax_state: CoaxLineState = CoaxLineState {
            line_state,
            inner_diameter: 0.00274,
            shield_diameter: 0.00739,
            dielectric_constant: 1.38 * FREE_SPACE_PERMITTIVITY,
            magnetic_permeability: 1.0 * FREE_SPACE_PERMEABILITY
        };
        coax_state.set_capacitance();
        coax_state.set_inductance();
        let resistivity_inner: f64 = 1.724e-8; // copper
        let resistivity_shield: f64 = 2.65e-8; // alu
        coax_state.set_resistance(resistivity_inner, resistivity_shield);
        coax_state.line_state.set_conductance();
        coax_state.line_state.set_impedance();
        coax_state.line_state.set_propagation_constant();
        dbg!(coax_state.line_state.z);
        dbg!(coax_state.get_loss_per_meter() * coax_state.line_state.length);
    }
}