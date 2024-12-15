pub mod coax_line;

mod trx_line {
    use num_complex::Complex;
    use crate::util::util::{hz_to_angular_freq, reflection_loss};

    pub struct TransmissionLineState {
        pub f: f64, // in hz
        pub length: f64, // in meters
        pub z: Complex<f64>, // impedance
        pub r: f64, // resistance ohms/meter
        pub l: f64, // inductance henries/meter
        pub c: f64, // capacitance farads/meter
        pub g: f64, // conductance siemens/meter
        gamma: Complex<f64> // propagation constant
    }

    pub enum TransmissionLineType {
        ideal,
    }

    impl TransmissionLineState {
        pub fn setup_line_state() {

        }

        pub fn set_conductance(&mut self, f: f64) {
            self.g = 1.0 / self.r;
        }
    
        pub fn set_impedance(&mut self, f: f64) -> Complex<f64> {
            let w: f64 = hz_to_angular_freq(f);
            self.z = (Complex::new(self.r, w * self.l) / Complex::new(self.g, w * self.c)).sqrt();
            self.z
        }
    
        pub fn set_propagation_constant(&mut self, f: f64) -> Complex<f64> {
            let w: f64 = hz_to_angular_freq(f);
            let z: Complex<f64> = Complex::new(self.r, w * self.l);
            let y: Complex<f64> = Complex::new(self.g, w * self.c);
            self.gamma = (z * y).sqrt();
            self.gamma
        }
    
        pub fn get_impedance_at_length(&mut self, z_load: Complex<f64>, f: f64, l: f64) -> Complex<f64> {
            let z_0: Complex<f64> = self.z;
            let x: Complex<f64> = (self.gamma * l).tanh();
            z_0 * ((z_load + (z_0 * x)) / (z_0 + (z_load * x)))
        }
    
        pub fn get_loss_at_freq(&mut self, f: f64) -> f64 {
            // db/meter
            0.0
        }
    
        pub fn total_loss(&mut self, z_load: Complex<f64>, z_source: Complex<f64>, f: f64, l: f64) -> f64 {
            reflection_loss(z_load, self.z) + (self.get_loss_at_freq(f) * l) + reflection_loss(self.get_impedance_at_length(z_load, f, l), z_source)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_trx_line() {
        
    }
}