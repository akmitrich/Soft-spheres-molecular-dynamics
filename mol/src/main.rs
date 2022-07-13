#![allow(unused)]
use d_vector::DVector;
use mol_job::job::{Job, JobSetup};
use mol_job::potential::LennardJones;

fn main() {
    let v = DVector::from([1., 0.22, 1e-6]) + DVector::from([0.4, 0.01, 0.0]);
    println!("Hello, {:?}", v.components());

    let mut j = create_job();
    j.run(100);
    println!("World 1: {:?}, time now {}", j, j.time_now());
    j.run(100);
    println!("World 2: {:?}, time now {}", j, j.time_now());
}

fn create_job() -> Job<3> {
    JobSetup::build().potential(LennardJones::new(3.)).job()
}
