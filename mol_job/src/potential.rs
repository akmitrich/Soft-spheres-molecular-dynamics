#![allow(unused, dead_code)]

use d_vector::{DVector, Real};

pub trait PotentialEnergy<const D: usize> {
    fn compute_forces(&self, pos: &[DVector<D>], acc: &mut [DVector<D>]);
    fn u_sum(&self) -> Real;
    fn virial_sum(&self) -> Real;
}

#[derive(Debug)]
pub struct LennardJones {
    r_cut: Real,
    u_sum: Real,
    v_sum: Real,
}

impl Default for LennardJones {
    fn default() -> Self {
        Self {
            r_cut: 2.5,
            u_sum: 0.0,
            v_sum: 0.0,
        }
    }
}

impl<const D: usize> PotentialEnergy<D> for LennardJones {
    fn compute_forces(&self, pos: &[DVector<D>], acc: &mut [DVector<D>]) {
        
    }

    fn u_sum(&self) -> Real {
        0.0
    }

    fn virial_sum(&self) -> Real {
        0.0
    }
}
