#![allow(unused, dead_code)]

use std::ops::AddAssign;

use d_vector::{DVector, Real};

#[derive(Debug)]
pub struct Region<const D: usize> {
    inner: DVector<D>,
}

impl<const D: usize> Region<D> {
    pub fn new(size: [Real; D]) -> Self {
        Self { inner: DVector::from(size) }
    }

    pub fn wrap(&self, position: &mut DVector<D>) {
        let mut shift = [0 as Real; D];
        for (i, s) in shift.iter_mut().enumerate() {
            if self.is_above(position, i) {
                s.add_assign(-self.inner.components()[i]);
            } else if self.is_below(position, i) {
                s.add_assign(self.inner.components()[i]);
            }
        }
        position.add_assign(DVector::from(shift));
    }

    fn is_above(&self, position: &DVector<D>, index: usize) -> bool {
        position.components()[index] >= self.inner.components()[index] / 2.
    }

    fn is_below(&self, position: &DVector<D>, index: usize) -> bool {
        position.components()[index] < -self.inner.components()[index] / 2.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrap() {
        let region = Region::new([1., 5.]);
        let mut p = DVector::from([1.5, -4.]);
        region.wrap(&mut p);
        assert_eq!(&[0.5, 1.], p.components());
    }
}