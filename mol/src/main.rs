#![allow(unused)]
use d_vector::DVector;
use mol_job::job::{Job, JobSetup};

fn main() {
    let v = DVector::from([1., 0.22, 1e-6]);
    println!("Hello, {:?}", v.components());

    let mut j: Job<3> = JobSetup::build()
        .job();
    j.run();
    println!("World, {:?}", j);
}
