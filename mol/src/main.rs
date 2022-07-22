#![allow(unused)]
use std::fs;

use d_vector::DVector;
use mol_job::initial_state;
use mol_job::job::{Job, JobSetup};
use mol_job::lennard_jones::LennardJones;
use mol_job::track::Track;

fn main() {
    let mut j = create_job();
    fs::write(
        "w2.txt",
        format!("Initial state: {:?}, time now {}", j, j.time_now()),
    );
    j.run(5);
    println!("Run 1 complete. vel_sum = {:?}", j.vel_sum());
    j.run(5);
    println!(
        "Run 2 complete. vel_sum = {:?}, {}",
        j.vel_sum(),
        j.vel_sum().length()
    );
}

fn create_job() -> Job<3> {
    let (boundaries, pos) = initial_state::cubic_lattice::<3>(10, 0.8);
    JobSetup::build()
        .state(Track::restore_from("track.txt"))
        .boundaries(boundaries)
        .init_pos(pos)
        .random_vel(1.)
        .potential(LennardJones::new(2.5))
        .job()
}
