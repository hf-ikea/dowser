use std::{fs::{self, OpenOptions}, io::{BufWriter, Write}, ops::{Index, IndexMut}};

trait Field {
    type Index;
    fn get(&self, idx: Self::Index) -> &f64;
    fn get_mut(&mut self, idx: Self::Index) -> &mut f64;
    /// Returns the largest index in every dimension that exists in this array
    fn max_index(&self) -> Self::Index;
}

pub struct Wire {
    array: Box<[f64]>,
}

impl Wire {
    pub fn new_zeroed(size: usize) -> Self {
        Self {
            array: vec![0.0; size].into_boxed_slice(),
        }
    }
    pub fn from_initial_state(initial: &[f64]) -> Self {
        Self {
            array: initial.to_vec().into_boxed_slice(),
        }
    }
    pub fn snapshot(field: &Wire, snapshot_idx: usize, new: bool) {
        let path = &format!("snap.csv");
        let mut file;
        if new {
            file = fs::File::create(path).unwrap();
            
        } else {
            file = OpenOptions::new().append(true).open(path).unwrap();
        }
        let mut buf_writer = BufWriter::new(file);
        for node in field.array.iter().enumerate() {
            writeln!(&mut buf_writer, "{snapshot_idx},{0},{1}", node.0, node.1).unwrap();
        }
    }
}

impl Field for Wire {
    type Index = usize;

    fn get(&self, idx: Self::Index) -> &f64 {
        &self.array[idx]
    }

    fn get_mut(&mut self, idx: Self::Index) -> &mut f64 {
        &mut self.array[idx]
    }
    fn max_index(&self) -> Self::Index {
        self.array.len() - 1
    }
}

impl Index<usize> for Wire {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.array[index]
    }
}

impl IndexMut<usize> for Wire {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.array[index]
    }
}

// basis
fn u(n: usize, x: f64, d_x: f64) -> f64 {
    let x_n: f64 = (n as f64 - 0.5) * d_x;
    if x > (x_n - d_x / 2.0) && x < (x_n + d_x / 2.0) {
        return 1.0;
    } else { return 0.0; }
}

// weighting
fn w(m: usize, x: f64, d_x: f64) -> f64 {
    let x_m: f64 = (m as f64 - 0.5) * d_x;
    if x > (x_m - d_x / 2.0) && x < (x_m + d_x / 2.0) {
        return 1.0;
    } else { return 0.0; }
}

fn w_w(m: usize, d_x: f64) -> f64 {
    let x_m: f64 = (m as f64 - 0.5) * d_x;
    let a_m: f64 = x_m - d_x;
    let b_m: f64 = x_m + d_x;
    let h_m: f64 = (b_m - a_m) / 10.0;

    let mut ww: f64 = (w(m, a_m, d_x) / 2.0) + (w(m, b_m, d_x) / 2.0);
    for i in 0..10 {
        ww += w(m, a_m + (i as f64 * h_m), d_x);
    }
    ww * h_m
}

// green
fn g(x: f64, x_p: f64) -> f64 {
    if x != x_p {
        return 1.0 / (x - x_p).abs();
    } else { println!("overlap in green!"); return 10.0; }
}

fn g_n(n: usize, x: f64, d_x: f64) -> f64 {
    let x_n: f64 = (n as f64 - 0.5) * d_x;
    let a_n: f64 = x_n - d_x / 2.0;
    let b_n: f64 = x_n + d_x / 2.0;
    let h_n: f64 = (b_n - a_n) / 10.0;

    let mut g_n: f64 = (u(n, a_n, d_x) * g(x, a_n) / 2.0) + (u(n, b_n, d_x) * g(x, b_n) / 2.0);
    for i in 0..10{
        g_n += u(n, a_n + (i as f64 * h_n), d_x) * g(x, a_n + (i as f64 * h_n));
    }
    g_n * h_n
}

fn w_g(m: usize, n: usize, d_x: f64) -> f64 {
    let x_m: f64 = (m as f64 - 0.5) * d_x;
    let a_m: f64 = x_m - d_x / 2.0;
    let b_m: f64 = x_m + d_x / 2.0;
    let h_m: f64 = (b_m - a_m) / 10.0;

    let mut wg: f64 = (w(m, a_m, d_x) * g_n(n, a_m, d_x) / 2.0) + (w(m, b_m, d_x) * g_n(n, b_m, d_x) / 2.0);
    for i in 0..10 {
        wg += w(m, a_m + (i as f64 * h_m), d_x)*g_n(n, a_m + (i as f64 * h_m), d_x);
    }
    wg * h_m
}

#[cfg(test)]
mod tests {
    use std::{f64::consts::PI, vec};

    use nalgebra::DMatrix;

    use crate::{consts::FREE_SPACE_PERMITTIVITY, mom::{w_g, w_w}, util::solve_square_matrix};

    use super::Wire;
    
    #[test]
    fn mom_wire() {
        const SEGMENTS: usize = 20;
        const WIRE_LENGTH: f64 = 1.0; // 1m
        const WIRE_RADIUS: f64 = 0.001; // 1mm
        const DELTA: f64 = WIRE_LENGTH / SEGMENTS as f64;
        const TEST_VOLTAGE: f64 = 1.0;

        let a_matrix: DMatrix<f64> = DMatrix::zeros(SEGMENTS, SEGMENTS).map_with_location(|n, m, _: f64| {
            if n == m {
                return 2.0 * (DELTA / WIRE_RADIUS).ln() * w_w(m, DELTA);
            } else {
                return w_g(m, n, DELTA);
            }
        });

        let b_matrix: Vec<f64> = (0..a_matrix.nrows()).map(|i| {
            4.0 * PI * FREE_SPACE_PERMITTIVITY * TEST_VOLTAGE * w_w(i, DELTA)
        }).collect();

        let solves = solve_square_matrix(a_matrix, b_matrix);
        dbg!(&solves);
    }
}
