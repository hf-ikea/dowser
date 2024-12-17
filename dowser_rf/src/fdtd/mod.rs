use std::ops::{Index, IndexMut};

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
#[cfg(test)]
mod tests {
    use crate::consts::FREE_SPACE_IMPEDANCE;

    use super::OneDField;

    #[test]
    fn fdtd_1d() {
        const SIZE: usize = 200;
        let mut e_field: OneDField = OneDField::new_zeroed(SIZE);
        let mut h_field: OneDField = OneDField::new_zeroed(SIZE);
        let max_time: usize = 250;

        for q in 0..max_time {
            for m in 0..(SIZE - 1) {
                h_field[m] = h_field[m] + (e_field[m + 1] - e_field[m]) / FREE_SPACE_IMPEDANCE;
            }

            for m in 1..SIZE {
                e_field[m] = e_field[m] + (h_field[m] - h_field[m - 1]) * FREE_SPACE_IMPEDANCE;
            }

            e_field[0] = (-(q as f64 - 30.0) * (q as f64 - 30.0) / 100.0).exp();

            println!("{0}", e_field[50]);
        }
    }
}
