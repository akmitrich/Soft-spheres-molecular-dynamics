#![allow(unused, dead_code)]

use std::{
    cell::{Cell, RefCell, RefMut},
    ops::AddAssign,
};

use crate::{boundaries::Region, potential::LennardJones, prop::TrivialProps, verlet};
use d_vector::{DVector, Real};

#[derive(Debug)]
pub struct Job<const D: usize> {
    pos: RefCell<Vec<DVector<D>>>,
    vel: RefCell<Vec<DVector<D>>>,
    acc: RefCell<Vec<DVector<D>>>,
    boundaries: Region<D>,
    potential: LennardJones,
    props: TrivialProps<D>,
    step_count: Cell<usize>,
    step_limit: usize,
    step_avg: usize,
    more_cycles: bool,
}

impl<const D: usize> verlet::State<D> for Job<D> {
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

impl<const D: usize> verlet::Config for Job<D> {
    fn step_begin(&self) {
        self.step_count.set(self.step_count.get() + 1);
    }

    fn need_avg(&self) -> bool {
        self.step_count.get() % self.step_avg == 0
    }
}

impl<const D: usize> Default for Job<D> {
    fn default() -> Self {
        Self {
            pos: RefCell::new(vec![]),
            vel: RefCell::new(vec![]),
            acc: RefCell::new(vec![]),
            boundaries: Region::new([50.; D]),
            potential: Default::default(),
            props: Default::default(),
            step_count: Cell::new(0),
            step_limit: 10,
            step_avg: 10,
            more_cycles: true,
        }
    }
}

impl<const D: usize> Job<D> {
    pub fn run(&mut self) {
        while self.more_cycles {
            verlet::single_step(self, self, &self.boundaries, &self.potential, &self.props);
            if self.step_count.get() >= self.step_limit {
                self.more_cycles = false;
            }
        }
    }
}
