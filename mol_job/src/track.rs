#![allow(unused, dead_code)]

use crate::state::MolecularState;
use d_vector::{DVector, Real};
use serde::{Deserialize, Serialize};
use std::{
    cell::{Cell, RefCell, RefMut},
    fs::{File, OpenOptions},
};

#[derive(Debug, Serialize)]
pub struct Track {
    time_now: Cell<Real>,
    pos: RefCell<Vec<DVector<3>>>,
    vel: RefCell<Vec<DVector<3>>>,
    acc: RefCell<Vec<DVector<3>>>,
    #[serde(skip_serializing)]
    output: File,
}

impl Default for Track {
    fn default() -> Self {
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .open("track.txt")
            .unwrap();
        Self {
            time_now: Default::default(),
            pos: Default::default(),
            vel: Default::default(),
            acc: Default::default(),
            output: file,
        }
    }
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
        self.time_now.set(time_now);
        let json = serde_json::to_string(self).unwrap();
        println!("{}", json);
    }
}
