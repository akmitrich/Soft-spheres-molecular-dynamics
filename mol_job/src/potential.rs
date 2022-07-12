#![allow(unused, dead_code)]

use d_vector::{DVector, Real};
use std::{cell::Cell, fmt::Debug};

pub trait PotentialEnergy<const D: usize>: Debug {
    fn compute_forces(&self, pos: &[DVector<D>], acc: &mut [DVector<D>]);
    fn u_sum(&self) -> Real;
    fn virial_sum(&self) -> Real;
}

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
    fn compute_forces(&self, pos: &[DVector<D>], acc: &mut [DVector<D>]) {}

    fn u_sum(&self) -> Real {
        self.u_sum.get()
    }

    fn virial_sum(&self) -> Real {
        self.v_sum.get()
    }
}
