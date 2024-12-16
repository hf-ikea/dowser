mod coax_line {
    use std::f64::consts::PI;

    use nalgebra::ComplexField;

    use crate::trx_line::trx_line::TransmissionLineState;

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

        pub fn get_loss_per_meter(&mut self) -> f64 {
            // db/meter
            8.686 * (self.line_state.r / (2.0 * self.line_state.z.re))
        }

        // henry/meter
        pub fn get_coax_inductance(permeability: f64, inner_conductor_diameter: f64, inner_shield_diameter: f64) -> f64 {
            (permeability / (2.0 * PI)) * ((inner_shield_diameter / inner_conductor_diameter).ln())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate:: util::util::{get_rf_resistance, get_skin_depth, FREE_SPACE_PERMEABILITY, FREE_SPACE_PERMITTIVITY};

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
        let mut coax_state: CoaxLineState = CoaxLineState::setup_coax_state();
        coax_state.inner_diameter = 0.00274;
        coax_state.shield_diameter = 0.00739;
        coax_state.dielectric_constant = 1.38 * FREE_SPACE_PERMITTIVITY;
        coax_state.magnetic_permeability = 1.0 * FREE_SPACE_PERMEABILITY;
        coax_state.set_capacitance();
        coax_state.set_inductance();
        coax_state.line_state.length = 100.0;
        dbg!(coax_state.line_state.l);
        let resistivity: f64 = 1.724e-8;
        let f: f64 = 3e7;
        let skin_depth: f64 = get_skin_depth(f, coax_state.magnetic_permeability, resistivity);
        coax_state.line_state.f = f;
        coax_state.line_state.r = get_rf_resistance(skin_depth, coax_state.shield_diameter, coax_state.line_state.length, resistivity);
        dbg!(coax_state.line_state.r);
        coax_state.line_state.set_conductance();
        coax_state.line_state.set_impedance();
        coax_state.line_state.set_propagation_constant();
        dbg!(coax_state.line_state.z);
        dbg!(coax_state.get_loss_per_meter() * coax_state.line_state.length);
    }
}