#![allow(unused, dead_code)]

use crate::{
    boundaries::BoundaryConditions, potential::PotentialEnergy, prop::Props, state::MolecularState,
};
use d_vector::{DVector, Real};
use std::{cell::RefMut, ops::AddAssign};

pub trait MolecularTimer<const D: usize> {
    fn step_begin(&self);
    fn delta_t(&self) -> Real {
        5e-3
    }
    fn step_complete(
        &self,
        state: &dyn MolecularState<D>,
        potential_energy: &dyn PotentialEnergy<D>,
    );
}

pub fn single_step<const D: usize>(
    delta_t: Real,
    state: &dyn MolecularState<D>,
    boundaries: &dyn BoundaryConditions<D>,
    potential_energy: &dyn PotentialEnergy<D>,
) {
    leapfrog_begin(
        delta_t,
        &mut state.get_pos(),
        &mut state.get_vel(),
        &state.get_acc(),
    );
    apply_boundary_conditions(boundaries, &mut state.get_pos());
    potential_energy.compute_forces(&state.get_pos(), &mut state.get_acc(), boundaries);
    leapfrog_end(delta_t, &mut state.get_vel(), &state.get_acc());
}

fn apply_boundary_conditions<const D: usize>(
    boundaries: &dyn BoundaryConditions<D>,
    pos: &mut [DVector<D>],
) {
    for position in pos.iter_mut() {
        boundaries.wrap(position)
    }
}

fn leapfrog_begin<const D: usize>(
    delat_t: Real,
    pos: &mut [DVector<D>],
    vel: &mut [DVector<D>],
    acc: &[DVector<D>],
) {
    assert_eq!(pos.len(), vel.len());
    calc_vel_for_half_step(delat_t, vel, acc);
    for (position, velocity) in pos.iter_mut().zip(vel.iter()) {
        position.add_assign(delat_t * velocity);
    }
}

fn leapfrog_end<const D: usize>(delat_t: Real, vel: &mut [DVector<D>], acc: &[DVector<D>]) {
    calc_vel_for_half_step(delat_t, vel, acc);
}

fn calc_vel_for_half_step<const D: usize>(
    delta_t: Real,
    vel: &mut [DVector<D>],
    acc: &[DVector<D>],
) {
    assert_eq!(vel.len(), acc.len());
    let half_delta_t = delta_t / 2.;
    for (velocity, acceleration) in vel.iter_mut().zip(acc.iter()) {
        velocity.add_assign(half_delta_t * acceleration);
    }
}
