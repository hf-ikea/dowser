#[cfg(test)]
mod tests {
    #[test]
    fn fdtd_1d() {
        let mut m: f64 = 0.0;
        let mut q: f64 = 0.0;
        let delta_x: f64 = 0.001;
        let delta_t: f64 = 0.001;
    }
}

trait EMField {
    type Index;
    fn get(&self, idx: Self::Index) -> f64;
    fn get_mut(&mut self, idx: Self::Index) -> &mut f64;
    /// Returns the largest index in every dimension that exists in this array
    fn max_index(&self) -> Self::Index;
}

pub struct OneDEMField {
    array: Box<[f64]>,
}

impl EMField for OneDEMField {
    type Index = usize;

    fn get(&self, idx: Self::Index) -> f64 {
        self.array[idx]
    }

    fn get_mut(&mut self, idx: Self::Index) -> &mut f64 {
        &mut self.array[idx]
    }
    fn max_index(&self) -> Self::Index {
        self.array.len() - 1
    }
}
