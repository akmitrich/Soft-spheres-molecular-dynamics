#![allow(unused, dead_code)]

use std::cell::Cell;

use d_vector::{DVector, Real};
use crate::{potential::PotentialEnergy, boundaries::BoundaryConditions};

#[derive(Debug)]
pub struct LennardJones {
    r_cut: Real,
    u_sum: Cell<Real>,
    v_sum: Cell<Real>,
}

impl Default for LennardJones {
    fn default() -> Self {
        Self {
            r_cut: 2.5,
            u_sum: Cell::new(0.0),
            v_sum: Cell::new(0.0),
        }
    }
}

impl<const D: usize> PotentialEnergy<D> for LennardJones {
    fn compute_forces(
        &self,
        pos: &[DVector<D>],
        acc: &mut [DVector<D>],
        boundaries: &dyn BoundaryConditions<D>,
    ) {
    }

    fn u_sum(&self) -> Real {
        self.u_sum.get()
    }

    fn virial_sum(&self) -> Real {
        self.v_sum.get()
    }
}

impl LennardJones {
    pub fn new(r_cut: Real) -> Self {
        Self {
            r_cut,
            ..Default::default()
        }
    }
}
