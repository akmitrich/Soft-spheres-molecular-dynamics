#![allow(unused)]
use std::fs;

use d_vector::DVector;
use mol_job::initial_state;
use mol_job::job::{Job, JobSetup};
use mol_job::lennard_jones::LennardJones;

fn main() {
    let v = DVector::from([1., 0.22, 1e-6]) + DVector::from([0.4, 0.01, 0.0]);
    println!("Hello, {:?}", v.components());

    let mut j = create_job();
    j.run(100);
    fs::write(
        "w1.txt",
        format!("World 1: {:?}, time now {}", j, j.time_now()),
    );
    println!("Run 1 complete. vel_sum = {:?}", j.vel_sum());
    j.run(100);
    fs::write(
        "w2.txt",
        format!("World 2: {:?}, time now {}", j, j.time_now()),
    );
    println!(
        "Run 2 complete. vel_sum = {:?}, {}",
        j.vel_sum(),
        j.vel_sum().length()
    );
}

fn create_job() -> Job<3> {
    let (boundaries, pos) = initial_state::cubic_lattice::<3>(1000, 0.8);
    JobSetup::build()
        .boundaries(boundaries)
        .init_pos(pos)
        .random_vel(1.)
        .potential(LennardJones::new(3.))
        .job()
}
