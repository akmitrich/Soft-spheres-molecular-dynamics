#![allow(unused, dead_code)]

use d_vector::DVector;
use crate::boundaries::Region;

#[derive(Debug)]
pub struct Job<const D: usize> {
    pos: Vec<DVector<D>>,
    vel: Vec<DVector<D>>,
    acc: Vec<DVector<D>>,
    boundaries: Region<D>,
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

impl<const D: usize> Default for Job<D> {
    fn default() -> Self {
        Self { 
            pos: Default::default(), 
            vel: Default::default(), 
            acc: Default::default(),
            boundaries: Region::new([50.; D]),
        }
    }
}