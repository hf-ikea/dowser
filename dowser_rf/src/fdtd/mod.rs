use std::ops::{Index, IndexMut};

trait EMField {
    type Index;
    fn get(&self, idx: Self::Index) -> &f64;
    fn get_mut(&mut self, idx: Self::Index) -> &mut f64;
    /// Returns the largest index in every dimension that exists in this array
    fn max_index(&self) -> Self::Index;
}
pub struct OneDEMField {
    array: Box<[f64]>,
}
impl OneDEMField {
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

impl EMField for OneDEMField {
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

impl Index<usize> for OneDEMField {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.array[index]
    }
}

impl IndexMut<usize> for OneDEMField {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.array[index]
    }
}
#[cfg(test)]
mod tests {
    use super::{EMField, OneDEMField};

    #[test]
    fn fdtd_1d() {
        let mut zeroed_field = OneDEMField::new_zeroed(10);
        let initialized_field = OneDEMField::from_initial_state(&[0.0, 1.0, 0.0]);
        // write to index 2
        zeroed_field[2] = 0.0;

        let mut m: f64 = 0.0;
        let mut q: f64 = 0.0;
        let delta_x: f64 = 0.001;
        let delta_t: f64 = 0.001;
    }
}
