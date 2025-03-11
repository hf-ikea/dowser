use num_complex::Complex;

#[derive(Debug, Clone, Copy)]
pub struct AntennaProperties {
    /// Current frequency in Hz
    pub frequency: f64,
    /// Orientation in radians (ie azimuth), respective to idk
    pub orientation: f64,
    /// The polarization of the antenna
    pub polarization: AntennaPolarization,
    /// Impedance of the source, typically going to be a feedline
    pub z_s: Complex<f64>,
}

#[derive(Debug, Clone, Copy)]
pub struct ModeledAntenna {
    pub antenna: AntennaProperties,
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

pub trait AntennaModel {
    fn model(&self, properties: AntennaProperties) -> ModeledAntenna;
}

impl ModeledAntenna {
    pub fn new(antenna: AntennaProperties, impedance: Complex<f64>) -> Self {
        Self {
            antenna,
            impedance,
        }
    }
}