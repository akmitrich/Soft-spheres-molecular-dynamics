#![allow(unused, dead_code)]

use d_vector::{DVector, Real};

#[derive(Debug)]
pub struct Region<const D: usize> {
    inner: DVector<D>,
}

impl<const D: usize> Region<D> {
    pub fn new(size: [Real; D]) -> Self {
        Self { inner: DVector::from(size) }
    }
}