mod coax_line {
    use std::f64::consts::PI;

    use crate::{trx_line::trx_line::TransmissionLineState, util::util::get_coax_inductance};

    pub struct CoaxLineState {
        line_state: TransmissionLineState,
        inner_diameter: f64, // outer diameter of inter conductor in meters
        shield_diameter: f64, // inner diameter of shield in meters
        dielectric_constant: f64,
        magnetic_permeability: f64, // relative
    }

    impl CoaxLineState {
        pub fn set_trx_line(&mut self, line: TransmissionLineState) {
            self.line_state = line;
        }

        pub fn set_capacitance(&mut self) {
            self.line_state.c = (2.0 * PI * self.dielectric_constant) / (self.shield_diameter / self.inner_diameter).ln();
        }

        pub fn set_inductance(&mut self) {
            self.line_state.l = get_coax_inductance(self.magnetic_permeability, self.inner_diameter, self.shield_diameter, self.line_state.length);
        }
    }
}