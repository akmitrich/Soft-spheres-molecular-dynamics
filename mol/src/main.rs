#![allow(unused)]
use std::fs;

use d_vector::DVector;
use mol_job::initial_state;
use mol_job::job::{Job, JobSetup};
use mol_job::lennard_jones::LennardJones;
use mol_job::track::Track;

fn main() {
    let mut job = create_job();
    fs::write(
        "w2.txt",
        format!("Initial state: {:?}, time now {}", job, job.time_now()),
    );
    job.run(5);
    println!(
        "Run 1 complete. vel_sum = {:?}. Must be zero.",
        job.vel_sum()
    );
    job.run(5);
    println!(
        "Run 2 complete. vel_sum = {:?}. Must be zero.",
        job.vel_sum()
    );
}

fn create_job() -> Job<3> {
    let (boundaries, pos) = initial_state::cubic_lattice::<3>(10, 0.8);
    let builder = JobSetup::build()
        .boundaries(boundaries)
        .potential(LennardJones::new(2.5));
    match Track::restore_from("track.txt") {
        Ok(restored_state) => builder.state(restored_state),
        Err(default_state) => builder.state(default_state).init_pos(pos).random_vel(1.),
    }
    .job()
}
