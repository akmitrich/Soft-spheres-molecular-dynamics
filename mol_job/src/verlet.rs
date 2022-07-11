#![allow(unused, dead_code)]

use std::{cell::RefMut, ops::AddAssign};
use d_vector::{DVector, Real};
use crate::potential::PotentialEnergy;

pub trait Config {
    fn step_begin(&self);
    fn delta_t(&self) -> Real {
        5e-3
    }
    fn need_avg(&self) -> bool;
}

pub trait State<const D: usize> {
    fn get_pos(&self) -> RefMut<Vec<DVector<D>>>;
    fn get_vel(&self) -> RefMut<Vec<DVector<D>>>;
    fn get_acc(&self) -> RefMut<Vec<DVector<D>>>;
}

pub trait BoundaryConditions<const D: usize> {
    fn wrap(&self, pos: &mut DVector<D>);
}

pub trait Props<const D: usize> {
    fn reset(&self);
    fn eval_props(&self, u: &dyn PotentialEnergy<D>, pos: &[DVector<D>], vel: &[DVector<D>]);
    fn accum_props(&self);
    fn avg_props(&self);
    fn summarize(&self) {}
}

pub fn single_step<const D: usize>(
    config: &dyn Config,
    state: &dyn State<D>,
    boundaries: &dyn BoundaryConditions<D>,
    potential_energy: &dyn PotentialEnergy<D>,
    props: &dyn Props<D>
) {
    let mut pos = state.get_pos();
    let mut vel = state.get_vel();
    let mut acc = state.get_acc();
    config.step_begin();

    leapfrog_begin(config.delta_t(), &mut pos, &mut vel, &acc);
    apply_boundary_conditions(boundaries, &mut pos);
    potential_energy.compute_forces(&pos, &mut acc);
    leapfrog_end(config.delta_t(), &mut vel, &acc);

    props.eval_props(potential_energy, &pos, &vel);
    props.accum_props();
    if config.need_avg() {
        props.avg_props();
        props.summarize();
        props.reset();
    }
}

fn apply_boundary_conditions<const D: usize>(b: &dyn BoundaryConditions<D>, p: &mut [DVector<D>]) {
    p
    .iter_mut()
    .for_each(|position| b.wrap(position));
}

fn leapfrog_begin<const D: usize>(dt: Real, p: &mut [DVector<D>], v: &mut [DVector<D>], a: &[DVector<D>]) {
    calc_vel_for_half_step(dt, v, a);
    p
        .iter_mut()
        .zip(v.iter())
        .for_each(|(p, v)| p.add_assign(dt * v));
}

fn leapfrog_end<const D: usize>(dt: Real, v: &mut [DVector<D>], a: &[DVector<D>]) {
    calc_vel_for_half_step(dt, v, a);
}

fn calc_vel_for_half_step<const D: usize>(dt: Real, v: &mut [DVector<D>], a: &[DVector<D>]) {
    let half_delta_t = dt / 2.;
    v
        .iter_mut()
        .zip(a.iter())
        .for_each(|(v, a)| v.add_assign(half_delta_t * a));
}
