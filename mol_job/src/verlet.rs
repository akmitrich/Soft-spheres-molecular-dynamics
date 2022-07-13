#![allow(unused, dead_code)]

use crate::{boundaries::BoundaryConditions, potential::PotentialEnergy, prop::Props};
use d_vector::{DVector, Real};
use std::{cell::RefMut, ops::AddAssign};

pub trait MolecularTimer<const D: usize> {
    fn step_begin(&self);
    fn delta_t(&self) -> Real {
        5e-3
    }
    fn step_complete(&self, state: &dyn MolecularState<D>, potential_energy: &dyn PotentialEnergy<D>);
}

pub trait MolecularState<const D: usize> {
    fn get_pos(&self) -> RefMut<Vec<DVector<D>>>;
    fn get_vel(&self) -> RefMut<Vec<DVector<D>>>;
    fn get_acc(&self) -> RefMut<Vec<DVector<D>>>;
}

pub fn single_step<const D: usize>(
    timer: &dyn MolecularTimer<D>,
    state: &dyn MolecularState<D>,
    boundaries: &dyn BoundaryConditions<D>,
    potential_energy: &dyn PotentialEnergy<D>,
) {
    timer.step_begin();

    leapfrog_begin(
        timer.delta_t(),
        &mut state.get_pos(),
        &mut state.get_vel(),
        &state.get_acc(),
    );
    apply_boundary_conditions(boundaries, &mut state.get_pos());
    potential_energy.compute_forces(&state.get_pos(), &mut state.get_acc());
    leapfrog_end(timer.delta_t(), &mut state.get_vel(), &state.get_acc());

    timer.step_complete(state, potential_energy);
}

fn apply_boundary_conditions<const D: usize>(
    boundaries: &dyn BoundaryConditions<D>,
    pos: &mut [DVector<D>],
) {
    pos.iter_mut()
        .for_each(|position| boundaries.wrap(position));
}

fn leapfrog_begin<const D: usize>(
    delat_t: Real,
    pos: &mut [DVector<D>],
    vel: &mut [DVector<D>],
    acc: &[DVector<D>],
) {
    assert_eq!(pos.len(), vel.len());
    calc_vel_for_half_step(delat_t, vel, acc);
    pos.iter_mut()
        .zip(vel.iter())
        .for_each(|(position, velocity)| position.add_assign(delat_t * velocity));
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
    vel.iter_mut()
        .zip(acc.iter())
        .for_each(|(velocity, acceleration)| velocity.add_assign(half_delta_t * acceleration));
}
