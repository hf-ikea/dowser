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
        pub gamma: Complex<f64> // propagation constant
    }

    impl TransmissionLineState {
        pub fn set_impedance(&mut self) {
            let w: f64 = hz_to_angular_freq(self.f);
            let x: Complex<f64> = Complex::new(self.r, w * self.l);
            let y: Complex<f64> = Complex::new(self.g, w * self.c);
            self.z = (x / y).sqrt();
        }

        pub fn set_conductance(&mut self) {
            //self.g = 10e-21; // i dont know???? this is conductance for PET ??
            self.g = hz_to_angular_freq(self.f) * self.c * 0.00015; // 0.00015 loss tangent for PTFE close enough???
        }
    
        pub fn set_propagation_constant(&mut self) {
            let w: f64 = hz_to_angular_freq(self.f);
            let z: Complex<f64> = Complex::new(self.r, w * self.l);
            let y: Complex<f64> = Complex::new(self.g, w * self.c);
            self.gamma = (z * y).sqrt();
        }
    
        pub fn get_impedance_at_length(&mut self, z_load: Complex<f64>, f: f64, l: f64) -> Complex<f64> {
            let z_0: Complex<f64> = self.z;
            let x: Complex<f64> = (self.gamma * l).tanh();
            z_0 * ((z_load + (z_0 * x)) / (z_0 + (z_load * x)))
        }
    
        pub fn total_match_loss(&mut self, z_load: Complex<f64>, z_source: Complex<f64>, f: f64, l: f64) -> f64 {
            reflection_loss(z_load, self.z) + reflection_loss(self.get_impedance_at_length(z_load, f, l), z_source)
        }
    }
}