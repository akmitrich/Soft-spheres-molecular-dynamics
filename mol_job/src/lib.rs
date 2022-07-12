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
        use potential::NoInteraction;
        let mut j: Job<3> = JobSetup::build()
        .delta_t(1e-3)
        .potential(NoInteraction::default())
        .job();
    j.run(100);
    assert_eq!(0.1, j.time_now())
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
