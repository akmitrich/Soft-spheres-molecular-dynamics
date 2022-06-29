#![allow(unused, dead_code)]

use d_vector::{DVector, Real};
use crate::boundaries::Region;

#[derive(Debug)]
pub struct Job<const D: usize> {
    pos: Vec<DVector<D>>,
    vel: Vec<DVector<D>>,
    acc: Vec<DVector<D>>,
    boundaries: Region<D>,
    delta_t: Real,
    t_now: Real,
    step_count: usize,
    step_limit: usize,
    step_avg: usize,
    more_cycles: bool,
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

    pub fn position_mut(&mut self, i: usize) -> &mut DVector<D> {
        self.pos.get_mut(i).unwrap()
    }

    pub fn velocity_mut(&mut self, i: usize) -> &mut DVector<D> {
        self.vel.get_mut(i).unwrap()
    }

    pub fn acceleration_mut(&mut self, i: usize) -> &mut DVector<D> {
        self.acc.get_mut(i).unwrap()
    }
}

impl<const D: usize> Default for Job<D> {
    fn default() -> Self {
        Self { 
            pos: vec![], 
            vel: vec![], 
            acc: vec![],
            boundaries: Region::new([50.; D]),
            delta_t: 1e-3,
            t_now: 0.,
            step_count: 0,
            step_limit: 10,
            step_avg: 10,
            more_cycles: true,
        }
    }
}

impl<const D: usize> Job<D> {
    pub fn n_mol(&self) -> usize {
        self.pos.len()
    }

    pub fn run(&mut self) {
        while self.more_cycles {
            self.single_step();
            if self.step_count >= self.step_limit {
                self.more_cycles = false;
            }
        }
    }
}

impl<const D: usize> Job<D> {
    fn reset_acc(&mut self) {
        self.acc.clear();
        for _ in 0..self.n_mol() {
            self.acc.push(Default::default());
        }
    }

    fn single_step(&mut self) {
        self.step_count += 1;
        self.t_now = self.step_count as Real * self.delta_t;
        self.apply_boundary_conditions();
    }

    fn apply_boundary_conditions(&mut self) {
        self.pos.iter_mut()
        .for_each(|p| self.boundaries.wrap(p));
    }
}