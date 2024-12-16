use crate::util::{hz_to_angular_freq, reflection_loss};
use num_complex::{Complex, ComplexFloat};

#[derive(Debug, Clone, Copy)]
pub struct FeedLine {
    /// Frequency in Hz
    pub frequency: f64,
    /// Length in meters
    pub length: f64,
    /// Complex load impedance
    pub z_l: Complex<f64>,
    /// Complex source impedance
    pub z_s: Complex<f64>,
}

#[derive(Debug, Clone, Copy)]
pub struct ModeledFeedLine {
    pub line: FeedLine,
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

pub trait Model {
    fn model(&self, properties: FeedLine) -> ModeledFeedLine;
}

impl ModeledFeedLine {
    pub fn new_from_irc(
        line: FeedLine,
        resistance: f64,
        inductance: f64,
        capacitance: f64,
    ) -> Self {
        // let g = 10e-21; // i dont know???? this is conductance for PET ??
        let g = hz_to_angular_freq(line.frequency) * capacitance * 0.00015; // 0.00015 loss tangent for PTFE close enough???,

        let w: f64 = hz_to_angular_freq(line.frequency);
        let x: Complex<f64> = Complex::new(resistance, w * inductance);
        let y: Complex<f64> = Complex::new(g, w * capacitance);
        let z = (x / y).sqrt();
        let gamma = (x * y).sqrt();
        Self {
            line,
            z,
            r: resistance,
            l: inductance,
            c: capacitance,
            g,
            gamma,
        }
    }
    pub fn get_impedance_at_length(&self) -> Complex<f64> {
        let z_0: Complex<f64> = self.z;
        let x: Complex<f64> = (self.gamma * self.l).tanh();
        z_0 * ((self.line.z_l + (z_0 * x)) / (z_0 + (self.line.z_l * x)))
    }
    /// Loss per meter in dB/meter
    pub fn get_loss_per_meter(&self) -> f64 {
        let resistive_loss: f64 = 4.34294 * self.r / self.z.abs();
        let dielectric_loss: f64 = 0.00409312451 * self.line.frequency * self.c * self.z.abs();
        dielectric_loss + resistive_loss
    }
    pub fn total_match_loss(&self) -> f64 {
        reflection_loss(self.line.z_l, self.z)
            + reflection_loss(self.get_impedance_at_length(), self.line.z_s)
    }
}
