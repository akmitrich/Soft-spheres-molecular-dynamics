#![allow(unused, dead_code)]

use std::cell::{RefCell, RefMut};

use d_vector::{DVector, Real};
use crate::state::MolecularState;

#[derive(Debug, Default)]
pub struct Track {
    pos: RefCell<Vec<DVector<3>>>,
    vel: RefCell<Vec<DVector<3>>>,
    acc: RefCell<Vec<DVector<3>>>,
    time_now: Real,
}

impl MolecularState<3> for Track {
    fn get_pos(&self) -> RefMut<Vec<DVector<3>>> {
        self.pos.borrow_mut()
    }

    fn get_vel(&self) -> RefMut<Vec<DVector<3>>> {
        self.vel.borrow_mut()
    }

    fn get_acc(&self) -> RefMut<Vec<DVector<3>>> {
        self.acc.borrow_mut()
    }

    fn sync(&self, time_now: Real) {

    }
}