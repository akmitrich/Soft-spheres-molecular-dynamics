#![allow(unused)]
use d_vector::DVector;
use mol_job::job::{Job, JobSetup};

fn main() {
    let v = DVector::from([1., 0.22, 1e-6]) + DVector::from([0.4, 0.01, 0.0]);
    println!("Hello, {:?}", v.components());

    let mut j: Job<3> = JobSetup::build()
        .step_limit(100)
        .step_avg(10)
        .job();
    j.run();
    println!("World, {:?}", j);
}
