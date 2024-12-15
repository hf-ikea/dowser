mod trx_line {
    use num_complex::Complex;
    use crate::util::util::{get_refl_coef, hz_to_angular_freq, reflection_loss};

    struct TransmissionLine {
        make: u32,
        z: Complex<f64>, // characteristic impedance
        length: f64, // in meters
        r: f64, // resistance ohms/meter
        l: f64, // inductance henries/meter
        c: f64, // capacitance farads/meter
        g: f64 // conductance siemens/meter
    }

    pub fn get_propagation_constant(line: TransmissionLine, f: f64) -> Complex<f64> {
        let w: f64 = hz_to_angular_freq(f);
        let z: Complex<f64> = Complex::new(line.r, w * line.l);
        let y: Complex<f64> = Complex::new(line.g, w * line.c);
        (z * y).sqrt()
    }

    pub fn get_impedance_at_length(line: TransmissionLine, z_load: Complex<f64>, f: f64, l: f64) -> Complex<f64> {
        let x = get_refl_coef(z_load, line.z);
        line.z * ((1.0 + x) / (1.0 - x))
    }

    pub fn get_loss_at_freq(line: &TransmissionLine, f: f64) -> f64 {
        // db/meter
        0.0
    }

    pub fn total_loss(line: TransmissionLine, z_load: Complex<f64>, z_source: Complex<f64>, f: f64, l: f64) -> f64 {
        reflection_loss(z_load, line.z) + (get_loss_at_freq(&line, f) * l) + reflection_loss(get_impedance_at_length(line, z_load, f, l), z_source)
    }
}