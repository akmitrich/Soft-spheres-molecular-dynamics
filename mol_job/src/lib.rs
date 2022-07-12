pub mod boundaries;
pub mod job;
pub mod potential;
pub mod prop;
pub mod verlet;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        use job::{Job, JobSetup};
        let mut j: Job<3> = JobSetup::build()
        .step_limit(100)
        .step_avg(10)
        .job();
    j.run();
    assert_eq!(0.5, j.time_now())
    }

    #[test]
    fn wrap() {
        use d_vector::DVector;
        use boundaries::{Region, BoundaryConditions};
        let region = Region::new([1., 5.]);
        let mut p = DVector::from([1.5, -4.]);
        region.wrap(&mut p);
        assert_eq!(&[0.5, 1.], p.components());
        let region = Region::new([1., 5.]);
        let mut p = DVector::from([0.2, -1.5]);
        region.wrap(&mut p);
        assert_eq!(&[0.2, -1.5], p.components());
    }
}
