use std::f64::consts::PI;

use crate::consts::FREE_SPACE_PERMEABILITY;

use crate::feed_line::{FeedLineProperties, FeedlineModel, ModeledFeedLine};

pub struct CoaxLineProperties {
    pub inner_diameter: f64,  // outer diameter of inter conductor in meters
    pub shield_diameter: f64, // inner diameter of shield in meters
    pub dielectric_constant: f64, // relative
    pub magnetic_permeability: f64, // relative
    pub resistivity_inner: f64,
    pub resistivity_shield: f64,
}

impl FeedlineModel for CoaxLineProperties {
    fn model(&self, properties: FeedLineProperties) -> ModeledFeedLine {
        // henry/meter
        fn get_coax_inductance(
            permeability: f64,
            inner_conductor_diameter: f64,
            inner_shield_diameter: f64,
        ) -> f64 {
            (permeability / (2.0 * PI)) * (inner_shield_diameter / inner_conductor_diameter).ln()
        }
        ModeledFeedLine::new_from_irc(
            properties,
            (properties.frequency * FREE_SPACE_PERMEABILITY / PI).sqrt()
                * ((self.resistivity_inner.sqrt() / self.inner_diameter)
                    + (self.resistivity_shield.sqrt() / self.shield_diameter)),
            get_coax_inductance(
                self.magnetic_permeability,
                self.inner_diameter,
                self.shield_diameter,
            ),
            (2.0 * PI * self.dielectric_constant)
                / (self.shield_diameter / self.inner_diameter).ln(),
        )
    }
}

#[cfg(test)]
mod tests {
    use num_complex::Complex;

    use crate::{
        consts::{FREE_SPACE_PERMEABILITY, FREE_SPACE_PERMITTIVITY},
        feed_line::FeedLineProperties,
    };

    use super::*;

    #[test]
    fn test_coax_line() {
        let properties: FeedLineProperties = FeedLineProperties {
            frequency: 100e6,
            length: 100.0,
            z_l: Complex::new(50.0, 0.0),
            z_s: Complex::new(50.0, 0.0),
        };
        let coax: CoaxLineProperties = CoaxLineProperties {
            inner_diameter: 0.00274,
            shield_diameter: 0.00739,
            dielectric_constant: 1.38 * FREE_SPACE_PERMITTIVITY,
            magnetic_permeability: 1.0 * FREE_SPACE_PERMEABILITY,
            resistivity_inner: 1.724e-8, // copper
            resistivity_shield: 2.65e-8, // alu
        };
        let model: ModeledFeedLine = coax.model(properties);

        println!(
            "Total line loss @ {0}MHz: {1}dB",
            model.line.frequency / 1e6,
            model.get_loss_per_meter() * model.line.length
        );
    }
}
