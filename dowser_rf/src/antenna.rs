use num_complex::Complex;

#[derive(Debug, Clone, Copy)]
pub struct CommonAntennaProperties {
    /// Current frequency in Hz
    pub frequency: f64,
    /// Orientation in radians (ie azumith), respective to idk
    pub orientation: f64,
    /// The polarization of the antenna
    pub polarization: AntennaPolarization,
}

#[derive(Debug, Clone, Copy)]
pub struct ModeledAntenna {
    /// Load impedance of the antenna
    pub impedance: Complex<f64>,
}

#[derive(Debug, Clone, Copy)]
pub enum AntennaPolarization {
    Horizontal,
    Vertical,
    Lefthand,
    Righthand,
}

pub trait Model {
    fn model(&self, properties: CommonAntennaProperties) -> ModeledAntenna;
}

impl ModeledAntenna {
    pub fn new_from_properties() -> Self {
        Self {
            impedance: Complex::new(50.0, 0.0)
        }
    }
}