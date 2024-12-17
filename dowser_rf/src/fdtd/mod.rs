use std::{f64::consts::PI, fs::{self, OpenOptions}, io::{BufReader, BufWriter, Write}, ops::{Index, IndexMut}, path::Path};

trait Field {
    type Index;
    fn get(&self, idx: Self::Index) -> &f64;
    fn get_mut(&mut self, idx: Self::Index) -> &mut f64;
    /// Returns the largest index in every dimension that exists in this array
    fn max_index(&self) -> Self::Index;
}
pub struct OneDField {
    array: Box<[f64]>,
}
impl OneDField {
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
    pub fn snapshot(field: &OneDField, snapshot_idx: usize, new: bool) {
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

impl Field for OneDField {
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

impl Index<usize> for OneDField {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.array[index]
    }
}

impl IndexMut<usize> for OneDField {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.array[index]
    }
}

pub fn harmonic_source(t: f64, x: f64, courant: f64, ppw: f64) -> f64 {
    f64::sin(2.0 * PI / ppw * (courant * t - x))
}

pub fn source_function(t: f64, x: f64, courant: f64) -> f64 {
    (-((t + 0.5 - x / courant / 100.0).powf(2.0))).exp()
}

#[cfg(test)]
mod tests {
    use crate::{consts::FREE_SPACE_IMPEDANCE, fdtd::{harmonic_source, source_function}};

    use super::OneDField;

    #[test]
    fn fdtd_1d() {
        const SIZE: usize = 200;
        const LOSS: f64 = 0.0253146;
        const LOSS_LAYER: usize = 180;
        const TFSF_BOUNDARY: usize = 50;
        let mut e_field: OneDField = OneDField::new_zeroed(SIZE);
        let mut h_field: OneDField = OneDField::new_zeroed(SIZE - 1);
        //let mut eps_r: OneDField = OneDField::from_initial_state(&[4.0; SIZE]);
        let mut eps_r: f64 = 9.0;
        let mut ceze: OneDField = OneDField::new_zeroed(SIZE);
        let mut cezh: OneDField = OneDField::new_zeroed(SIZE);
        let mut chyh: OneDField = OneDField::new_zeroed(SIZE - 1);
        let mut chye: OneDField = OneDField::new_zeroed(SIZE - 1);
        let courant: f64 = 1.0;
        let max_time: usize = 250;
        let ppw: f64 = 40.0;

        let mut x: f64 = (cezh[0] * chye[0]).sqrt();
        let abc_left = (x - 1.0) / (x + 1.0);
        x = (cezh[SIZE - 1] * chye[SIZE - 1]).sqrt();
        let abc_right = (x - 1.0) / (x + 1.0);
        let mut old_left: f64 = 0.0;
        let mut old_right: f64 = 0.0;

        for m in 1..SIZE {
            if m < 100 {
                ceze[m] = 1.0;
                cezh[m] = FREE_SPACE_IMPEDANCE;
            } else {
                ceze[m] = 1.0;
                cezh[m] = FREE_SPACE_IMPEDANCE / eps_r;
            }
        }

        for m in 1..(SIZE - 1) {
            chyh[m] = 1.0;
            chye[m] = 1.0 / FREE_SPACE_IMPEDANCE;
        }

        for q in 0..max_time {
            // ABC for h[size-1]
            h_field[SIZE - 1] = h_field[SIZE - 2];
            for m in 0..(SIZE - 1) {
                h_field[m] = chyh[m] * h_field[m] + chye[m] * (e_field[m + 1] - e_field[m]);
            }
            h_field[TFSF_BOUNDARY] -= source_function(q as f64, 0.0, courant);
            e_field[TFSF_BOUNDARY + 1] += source_function(q as f64 + 0.5, -0.5, courant);
            // ABS for e[0]
            e_field[0] = e_field[1];
            for m in 1..SIZE {
                e_field[m] = ceze[m] * e_field[m] + cezh[m] * (h_field[m] - h_field[m - 1]);
            }

            e_field[0] = old_left + abc_left * (e_field[1] - e_field[0]);
            old_left = e_field[1];
            e_field[SIZE - 1] = old_right + abc_right * (e_field[SIZE - 2] - e_field[SIZE - 1]);
            old_right = e_field[SIZE - 2];

            OneDField::snapshot(&e_field, q, q == 0);
        }
    }
}
