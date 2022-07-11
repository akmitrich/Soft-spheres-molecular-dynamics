#![allow(unused, dead_code)]

use std::ops::{AddAssign, SubAssign};
use d_vector::{DVector, Real};
use crate::verlet;

pub trait BoundaryConditions<const D: usize> {
    fn wrap(&self, pos: &mut DVector<D>);
}

#[derive(Debug)]
pub struct Region<const D: usize> {
    inner: DVector<D>,
}

impl<const D: usize> Region<D> {
    pub fn new(size: [Real; D]) -> Self {
        Self {
            inner: DVector::from(size),
        }
    }

    fn is_above(&self, position: &DVector<D>, index: usize) -> bool {
        position.components()[index] >= self.inner.components()[index] / 2.
    }

    fn is_below(&self, position: &DVector<D>, index: usize) -> bool {
        position.components()[index] < -self.inner.components()[index] / 2.
    }
}

impl<const D: usize> BoundaryConditions<D> for Region<D> {
    fn wrap(&self, position: &mut DVector<D>) {
        let mut shift = [0 as Real; D];
        for (i, s) in shift.iter_mut().enumerate() {
            if self.is_above(position, i) {
                s.sub_assign(self.inner.components()[i]);
            } else if self.is_below(position, i) {
                s.add_assign(self.inner.components()[i]);
            }
        }
        position.add_assign(DVector::from(shift));
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
        let region = Region::new([1., 5.]);
        let mut p = DVector::from([0.2, -1.5]);
        region.wrap(&mut p);
        assert_eq!(&[0.2, -1.5], p.components());
    }
}
