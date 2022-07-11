#![allow(unused, dead_code)]

use d_vector::DVector;
use crate::potential::PotentialEnergy;

pub trait Props<const D: usize> {
    fn reset(&self);
    fn eval_props(&self, u: &dyn PotentialEnergy<D>, pos: &[DVector<D>], vel: &[DVector<D>]);
    fn accum_props(&self);
    fn avg_props(&self);
    fn summarize(&self) {}
}

#[derive(Debug, Default)]
pub struct TrivialProps<const D: usize>;

impl<const D: usize> Props<D> for TrivialProps<D> {
    fn reset(&self) {
        
    }

    fn eval_props(&self, u: &dyn PotentialEnergy<D>, pos: &[DVector<D>], vel: &[DVector<D>]) {
        
    }

    fn accum_props(&self) {
        
    }

    fn avg_props(&self) {
        
    }
}