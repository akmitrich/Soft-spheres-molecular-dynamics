#[allow(unused, dead_code)]

use d_vector::DVector;

struct Job<const D: usize> {
    pos: Vec<DVector<D>>,
    vel: Vec<DVector<D>>,
    acc: Vec<DVector<D>>,
}

impl<const D: usize> Job<D> {
    pub fn position(&self, i: usize) -> &DVector<D> {
        self.pos.get(i).unwrap()
    }

    pub fn velocity(&self, i: usize) -> &DVector<D> {
        self.vel.get(i).unwrap()
    }

    pub fn acceleration(&self, i: usize) -> &DVector<D> {
        self.acc.get(i).unwrap()
    }
}