#![allow(unused, dead_code)]

use d_vector::DVector;

pub trait PotentialEnergy<const D: usize> {
    fn compute_forces(&self, pos: &[DVector<D>], acc: &mut [DVector<D>]);
    fn u_sum(&self);
    fn virial_sum(&self);
}

