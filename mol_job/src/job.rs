#![allow(unused, dead_code)]

use std::{
    cell::{Cell, RefCell, RefMut},
    ops::AddAssign,
};

use crate::{
    boundaries::{BoundaryConditions, Region},
    potential::{LennardJones, PotentialEnergy},
    prop::{TrivialProps, Props},
    verlet,
};
use d_vector::{DVector, Real};

#[derive(Debug)]
pub struct Job<const D: usize> {
    pos: RefCell<Vec<DVector<D>>>,
    vel: RefCell<Vec<DVector<D>>>,
    acc: RefCell<Vec<DVector<D>>>,
    boundaries: Box<dyn BoundaryConditions<D>>,
    potential: Box<dyn PotentialEnergy<D>>,
    props: Box<dyn Props<D>>,
    step_count: Cell<usize>,
    step_limit: usize,
    delta_t: Real,
    more_cycles: bool,
}

impl<const D: usize> verlet::MolecularState<D> for Job<D> {
    fn get_pos(&self) -> RefMut<Vec<DVector<D>>> {
        self.pos.borrow_mut()
    }

    fn get_vel(&self) -> RefMut<Vec<DVector<D>>> {
        self.vel.borrow_mut()
    }

    fn get_acc(&self) -> RefMut<Vec<DVector<D>>> {
        self.acc.borrow_mut()
    }
}

impl<const D: usize> verlet::MolecularTimer for Job<D> {
    fn step_begin(&self) {
        self.step_count.set(self.step_count.get() + 1);
    }

    fn step_count(&self) -> usize {
        self.step_count.get()
    }

    fn delta_t(&self) -> Real {
        self.delta_t
    }
}

impl<const D: usize> Default for Job<D> {
    fn default() -> Self {
        Self {
            pos: RefCell::new(vec![]),
            vel: RefCell::new(vec![]),
            acc: RefCell::new(vec![]),
            boundaries: Box::new(Region::new([50.; D])),
            potential: Box::new(LennardJones::default()),
            props: Box::new(TrivialProps::default()),
            step_count: Cell::new(0),
            step_limit: 10,
            delta_t: 0.005,
            more_cycles: true,
        }
    }
}

impl<const D: usize> Job<D> {
    pub fn run(&mut self) {
        while self.more_cycles {
            verlet::single_step(
                self,
                self,
                self.boundaries.as_ref(),
                self.potential.as_ref(),
                self.props.as_ref(),
            );
            if self.step_count.get() >= self.step_limit {
                self.more_cycles = false;
            }
        }
    }

    pub fn time_now(&self) -> Real {
        verlet::MolecularTimer::delta_t(self) * self.step_count.get() as Real
    }
}

pub struct JobSetup<const D: usize>(Job<D>);

impl<const D: usize> JobSetup<D> {
    pub fn build() -> Self {
        let mut job = Job::default();
        Self(job)
    }

    pub fn step_limit(mut self, limit: usize) -> Self {
        self.0.step_limit = limit;
        self
    }

    pub fn delta_t(mut self, dt: Real) -> Self {
        self.0.delta_t = dt;
        self
    }

    pub fn potential(mut self, potential: impl PotentialEnergy<D> + 'static) -> Self {
        self.0.potential = Box::new(potential);
        self
    }

    pub fn job(self) -> Job<D> {
        self.0
    }
}