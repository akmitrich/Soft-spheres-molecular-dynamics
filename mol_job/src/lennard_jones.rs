#![allow(unused, dead_code)]

use std::{cell::Cell, fs};

use d_vector::{DVector, Real, reset_array};
use crate::{potential::PotentialEnergy, boundaries::BoundaryConditions};

#[derive(Debug)]
pub struct LennardJones {
    r_cut: Real,
    u_sum: Cell<Real>,
    v_sum: Cell<Real>,
    n: Cell<usize>
}

impl Default for LennardJones {
    fn default() -> Self {
        Self {
            r_cut: 2.5,
            u_sum: Cell::new(0.0),
            v_sum: Cell::new(0.0),
            n: Cell::new(0)
        }
    }
}

impl<const D: usize> PotentialEnergy<D> for LennardJones {
    fn compute_forces(
        &self,
        pos: &[DVector<D>],
        acc: &mut [DVector<D>],
        boundaries: &dyn BoundaryConditions<D>,
    ) {
        let n_mol = pos.len();
        assert!(n_mol > 0);
        assert_eq!(n_mol, acc.len());

        let rr_cut = self.r_cut * self.r_cut;
        reset_array(acc);
        let mut u_sum = 0 as Real;
        let mut v_sum = 0 as Real;

        for j1 in 0..(n_mol - 1) {
            for j2 in (j1 + 1)..n_mol {
                let mut dr = &pos[j1] - &pos[j2];
                boundaries.wrap(&mut dr);
                let rr = dr.square_length();
                if rr < rr_cut {
                    let rri = 1. / rr;
                    let rri3 = rri * rri * rri;

                    let force_value = 48. * rri3 * (rri3 - 0.5) * rri;
                    let force = force_value * dr;

                    acc[j1] += &force;
                    acc[j2] -= &force;

                    u_sum += 4. * rri3 * (rri3 - 1.) + 1.;
                    v_sum += force_value * rr;
                }
            }
        }
        self.u_sum.set(u_sum);
        self.v_sum.set(v_sum);

        self.n.set(self.n.get() + 1);
        println!("{}. u = {}, v = {}", self.n.get(), u_sum, v_sum);
    }

    fn u_sum(&self) -> Real {
        self.u_sum.get()
    }

    fn virial_sum(&self) -> Real {
        self.v_sum.get()
    }
}

impl LennardJones {
    pub fn new(r_cut: Real) -> Self {
        Self {
            r_cut,
            ..Default::default()
        }
    }
}
