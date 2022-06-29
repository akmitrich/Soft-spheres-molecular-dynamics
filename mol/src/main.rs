#![allow(unused)]
use d_vector::DVector;
use mol_job::job::Job;

fn main() {
    let v = DVector::from([1., 0.22, 1e-6]);
    println!("Hello, {:?}", v.components());

    let j: Job<3> = Job::default();
    println!("World, {:?}", j);
}
