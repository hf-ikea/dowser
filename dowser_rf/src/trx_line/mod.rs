pub mod coax_line;
use crate::util::{hz_to_angular_freq, reflection_loss};
use num_complex::{Complex, ComplexFloat};

#[derive(Debug, Clone, Copy)]
struct TransmissionLineProperties {
    /// Frequency in Hz
    frequency: f64,
    /// Length in meters
    length: f64,
    /// Complex load impedance
    z_l: Complex<f64>,
    /// Complex source impedance
    z_s: Complex<f64>,
}

#[derive(Debug, Clone, Copy)]
struct ModeledLine {
    pub line_properties: TransmissionLineProperties,
    /// Complex impedance
    pub z: Complex<f64>,
    /// Resistance in ohms/meter
    pub r: f64,
    /// Inductance in henries/meter
    pub l: f64,
    /// Capacitance in farads/meter
    pub c: f64,
    /// Conductance in siemens/meter
    pub g: f64,
    /// Propagation constant
    pub gamma: Complex<f64>,
}

trait Model {
    fn model(&self, properties: TransmissionLineProperties) -> ModeledLine;
}

impl ModeledLine {
    fn new(
        properties: TransmissionLineProperties,
        resistance: f64,
        inductance: f64,
        capacitance: f64,
    ) -> Self {
        // let g = 10e-21; // i dont know???? this is conductance for PET ??
        let g = hz_to_angular_freq(properties.frequency) * capacitance * 0.00015; // 0.00015 loss tangent for PTFE close enough???,

        let w: f64 = hz_to_angular_freq(properties.frequency);
        let x: Complex<f64> = Complex::new(resistance, w * inductance);
        let y: Complex<f64> = Complex::new(g, w * capacitance);
        let z = (x / y).sqrt();
        let gamma = (x * y).sqrt();
        Self {
            line_properties: properties,
            z,
            r: resistance,
            l: inductance,
            c: capacitance,
            g,
            gamma,
        }
    }
    pub fn get_impedance_at_length(&self, z_load: Complex<f64>, f: f64, l: f64) -> Complex<f64> {
        let z_0: Complex<f64> = self.z;
        let x: Complex<f64> = (self.gamma * l).tanh();
        z_0 * ((z_load + (z_0 * x)) / (z_0 + (z_load * x)))
    }
    /// Loss per meter in dB/meter
    pub fn get_loss_per_meter(&self) -> f64 {
        let resistive_loss: f64 = 4.34294 * self.r / self.z.abs();
        let dielectric_loss: f64 =
            0.00409312451 * self.line_properties.frequency * self.c * self.z.abs();
        dielectric_loss + resistive_loss
    }
    pub fn total_match_loss(&self) -> f64 {
        reflection_loss(self.line_properties.z_l, self.z)
            + reflection_loss(
                self.get_impedance_at_length(
                    self.line_properties.z_l,
                    self.line_properties.frequency,
                    self.line_properties.length,
                ),
                self.line_properties.z_s,
            )
    }
}