#![allow(unused, dead_code)]

use std::{
    cell::{Cell, RefCell, RefMut},
    ops::AddAssign,
};

use crate::{
    boundaries::{BoundaryConditions, Region},
    potential::{LennardJones, PotentialEnergy},
    prop::{Props, TrivialProps},
    verlet::{self, MolecularTimer},
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

impl<const D: usize> verlet::MolecularTimer<D> for Job<D> {
    fn step_begin(&self) {
        self.step_count.set(self.step_count() + 1);
    }

    fn delta_t(&self) -> Real {
        self.delta_t
    }

    fn step_complete(
        &self,
        state: &dyn verlet::MolecularState<D>,
        potential_energy: &dyn PotentialEnergy<D>,
    ) {
        self.props
            .eval_props(potential_energy, &state.get_pos(), &state.get_vel());
        self.props.accum_props();
        if self.props.need_avg(self.step_count()) {
            self.props.avg_props();
            self.props.summarize();
            self.props.reset();
        }
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
            delta_t: 0.005,
            more_cycles: true,
        }
    }
}

impl<const D: usize> Job<D> {
    pub fn run(&mut self, steps: usize) -> usize {
        self.more_cycles = true;
        let step_limit = self.step_count() + steps;
        while self.more_cycles {
            verlet::single_step(
                self,
                self,
                self.boundaries.as_ref(),
                self.potential.as_ref(),
            );
            if self.step_count() >= step_limit {
                self.more_cycles = false;
            }
        }
        self.step_count() - step_limit
    }

    pub fn time_now(&self) -> Real {
        verlet::MolecularTimer::delta_t(self) * self.step_count() as Real
    }

    pub fn step_count(&self) -> usize {
        self.step_count.get()
    }

    pub fn vel_sum(&self) -> DVector<D> {
        let mut result = DVector::default();
        for velocity in self.vel.borrow().iter() {
            result += velocity;
        }
        result
    }
}

pub struct JobSetup<const D: usize>(Job<D>);

impl<const D: usize> JobSetup<D> {
    pub fn build() -> Self {
        let mut job = Job::default();
        Self(job)
    }

    pub fn delta_t(mut self, dt: Real) -> Self {
        self.0.delta_t = dt;
        self
    }

    pub fn potential(mut self, potential: impl PotentialEnergy<D> + 'static) -> Self {
        self.0.potential = Box::new(potential);
        self
    }

    pub fn props(mut self, props: impl Props<D> + 'static) -> Self {
        self.0.props = Box::new(props);
        self
    }

    pub fn boundaries(mut self, boundaries: impl BoundaryConditions<D> + 'static) -> Self {
        self.0.boundaries = Box::new(boundaries);
        self
    }

    pub fn init_pos(mut self, pos: Vec<DVector<D>>) -> Self {
        let n_mol = pos.len();
        self.0.pos = RefCell::new(pos);
        self.0.vel = RefCell::new(vec![DVector::default(); n_mol]);
        self.0.acc = RefCell::new(vec![DVector::default(); n_mol]);
        self
    }

    pub fn random_vel(mut self, temperature: Real) -> Self {
        let n_mol = self.0.vel.borrow().len();
        let vel_mag = (temperature * (D as Real) * (1. - 1. / (n_mol as Real))).sqrt();
        for v in self.0.vel.borrow_mut().iter_mut() {
            *v = vel_mag * DVector::random_vector();
        }
        let sum = self.0.vel_sum();
        let k = -1. / n_mol as Real;
        for v in self.0.vel.borrow_mut().iter_mut() {
            v.add_assign(k * &sum);
        }
        self
    }

    pub fn job(self) -> Job<D> {
        self.0
    }
}
