#![allow(unused)]
use d_vector::DVector;
use mol_job::job::{Job, JobSetup};
use mol_job::potential::LennardJones;

fn main() {
    let v = DVector::from([1., 0.22, 1e-6]) + DVector::from([0.4, 0.01, 0.0]);
    println!("Hello, {:?}", v.components());

    let mut j = create_job();
    j.run();
    println!("World, {:?}", j);
}

fn create_job() -> Job<3> {
    JobSetup::build()
        .step_limit(100)
        .potential(LennardJones::new(3.))
        .job()
}
