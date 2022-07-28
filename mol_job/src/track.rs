#![allow(unused, dead_code)]

use crate::state::{MolecularState, State};
use d_vector::{DVector, Real};
use std::{
    cell::{Cell, RefCell, RefMut},
    fs::{File, OpenOptions},
    io::{Write, BufReader, BufRead}, path::Path,
};

#[derive(Debug)]
pub struct Track {
    inner: State<3>,
    output: RefCell<File>,
}

impl Default for Track {
    fn default() -> Self {
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open("track.txt")
            .unwrap();
        Self {
            inner: State::default(),
            output: RefCell::new(file),
        }
    }
}

impl MolecularState<3> for Track {
    fn get_pos(&self) -> RefMut<Vec<DVector<3>>> {
        self.inner.get_pos()
    }

    fn get_vel(&self) -> RefMut<Vec<DVector<3>>> {
        self.inner.get_vel()
    }

    fn get_acc(&self) -> RefMut<Vec<DVector<3>>> {
        self.inner.get_acc()
    }

    fn sync(&self, time_now: Real) {
        let json = serde_json::to_string(&self.inner).unwrap();
        writeln!(self.output.borrow_mut(), "{}. {}", time_now, json);
    }
}

impl Track {
    pub fn restore_from<P: AsRef<Path>>(path: P) -> Self {
        match OpenOptions::new()
            .read(true)
            .open(path) {
                Ok(input) => {
                    let mut last_line = String::new();
                    for line in BufReader::new(input).lines().flatten() {
                        last_line = line;
                    }
                    let mut data = String::new();
                    for item in last_line.split(". ") {
                        data = String::from(item);
                    }
                    let data: State<3> = serde_json::from_str(&data).unwrap();
                    println!("Restore Track from: {:?}", data);
                },
                Err(_) => todo!(),
            }
        Self::default()
    }
}