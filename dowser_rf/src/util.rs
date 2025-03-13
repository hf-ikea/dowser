use std::{f64::consts::PI, vec};

use nalgebra::{stack, DMatrix, DVector, Dyn, Matrix, OMatrix, VecStorage};
use num_complex::{Complex, ComplexFloat};

pub fn coth(x: f64) -> f64 {
    if x == 0.0 {
        return f64::NAN;
    } 
    x.cosh() / x.sinh()
}

/// Computes sin^2(x)
pub fn sin2(x: f64) -> f64 {
    f64::sin(x).powi(2)
}

/// in db
pub fn reflection_loss(z_load: Complex<f64>, z_source: Complex<f64>) -> f64 {
    -20.0 * f64::log10(((z_load - z_source) / (z_load + z_source)).abs())
}

pub fn get_refl_coef(z_load: Complex<f64>, z_source: Complex<f64>) -> Complex<f64> {
    (z_load - z_source) / (z_load + z_source)
}

pub fn swr(z_load: Complex<f64>, z_source: Complex<f64>) -> f64 {
    let refl_abs: f64 = get_refl_coef(z_load, z_source).abs();
    (1.0 + refl_abs) / (1.0 - refl_abs)
}

pub fn hz_to_angular_freq(f: f64) -> f64 {
    2.0 * PI * f
}

pub fn get_skin_depth(f: f64, permeability: f64, resistivity: f64) -> f64 {
    (resistivity / (PI * f * permeability)).sqrt()
}

/// in ohm-meters
pub fn get_rf_resistance(skin_depth: f64, diameter: f64, resistivity: f64) -> f64 {
    (resistivity) / (PI * skin_depth * diameter)
}

pub fn gaussian_elimination(mut augmented: DMatrix<f64>) -> DMatrix<f64> {
    let mut h: usize = 0; // pivot row
    let mut k: usize = 0; // pivot column
    let mut swap_pos: Vec<(usize, usize)> = Vec::new();

    let epsilon: f64 = 1e-15;
    while h < augmented.nrows() && k < augmented.ncols() {
        let mut i_max: usize = 0;
        for i in 0..augmented.nrows() {
            if augmented[(i_max, k)] < augmented[(i, k)] {
                i_max = i;
            }
        }
        if augmented[(i_max, k)] == 0.0 || augmented[(i_max, k)].abs() <= epsilon {
            k += 1;
        } else {
            swap_pos.push((h, i_max));
            augmented.swap_rows(h, i_max);
            for i in (h + 1)..augmented.nrows() {
                let f: f64 = augmented[(i, k)] / augmented[(h, k)];
                augmented[(i, k)] = 0.0;
                for j in (k + 1)..augmented.ncols() {
                    augmented[(i , j)] = augmented[(i, j)] - augmented[(h, j)] * f;
                }
            }
            h += 1;
            k += 1;
        }
    }

    for (h, i_max) in swap_pos {
        augmented.swap_rows(h, i_max);
    }
    augmented
}

pub fn solve_square_matrix(a_matrix: DMatrix<f64>, b_matrix: Vec<f64>) -> Vec<f64> {
    let elim: DMatrix<f64> = gaussian_elimination(stack![a_matrix, DVector::from_vec(b_matrix)]);
    let mut x: Vec<f64> = vec![0.0; elim.nrows()];

    for i in (0..elim.nrows()).rev() {
        let mut sum: f64 = 0.0;
        for j in i..(elim.ncols() - 1) {
            sum += elim[(i, j)] * x[j];
        }
        x[i] = (elim[(i, elim.ncols() - 1)] - sum) / elim[(i, i)];
    }
    x
}

#[cfg(test)]
mod tests {
    use nalgebra::{DMatrix, DVector};

    use crate::{consts::FREE_SPACE_PERMEABILITY, util::{get_rf_resistance, get_skin_depth, solve_square_matrix}};

    #[test]
    fn test_rf_resistance() {
        let resistivity: f64 = 2.44e-8; // gold
        let permeability: f64 = 1.0 * FREE_SPACE_PERMEABILITY;
        let f: f64 = 50.0;
        let skin_depth: f64 = get_skin_depth(f, permeability, resistivity);
        dbg!(skin_depth);
        let rf_resistance: f64 = get_rf_resistance(skin_depth, 0.033, resistivity);
        dbg!(rf_resistance);
    }

    #[test]
    fn test_gaussian() {
        let a_matrix: DMatrix<f64> = DMatrix::from_vec(3, 3, vec![ // watch out! apparently from vec fills column by column!!
            3.0, 2.0, 1.0,
            2.0, 6.0, 1.0,
            4.0, 5.0, 1.0,
        ]);
        let b_matrix: Vec<f64> = vec![
            19.0,
            29.0,
            6.0,
        ];
        let x: Vec<f64> = solve_square_matrix(a_matrix, b_matrix);

        assert!((3.0 - x[2]).abs() < 1e-10);
        assert!((2.0 - x[1]).abs() < 1e-10);
        assert!((1.0 - x[0]).abs() < 1e-10);
    }
}
