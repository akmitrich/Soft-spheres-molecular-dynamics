use std::cell::RefMut;
use std::ops::DerefMut;

use d_vector::{DVector, Real};
use mol_job::boundaries::Region;
use mol_job::job::JobSetup;
use mol_job::lennard_jones::LennardJones;
use mol_job::state::{MolecularState, State};

#[derive(Debug)]
struct TwoState {
    inner: State<2>,
}

impl Default for TwoState {
    fn default() -> Self {
        let inner = State::default();
        let pos = vec![DVector::from([-1., -1.]), DVector::from([1., 1.])];
        let vel = vec![DVector::from([1., 1.]), DVector::from([-1., -1.])];
        *(inner.get_pos().deref_mut()) = pos;
        *(inner.get_vel().deref_mut()) = vel;
        *(inner.get_acc().deref_mut()) = vec![DVector::default(); 2];
        Self { inner }
    }
}

impl MolecularState<2> for TwoState {
    fn get_pos(&self) -> RefMut<Vec<DVector<2>>> {
        self.inner.get_pos()
    }

    fn get_vel(&self) -> RefMut<Vec<DVector<2>>> {
        self.inner.get_vel()
    }

    fn get_acc(&self) -> RefMut<Vec<DVector<2>>> {
        self.inner.get_acc()
    }

    fn sync(&self, time_now: Real) {
        let pos_ref = self.get_pos();
        let vel_ref = self.get_vel();
        let acc_ref = self.get_acc();
        let left_position = pos_ref.get(0).unwrap();
        let right_position = pos_ref.get(1).unwrap();
        let left_velocity = vel_ref.get(0).unwrap();
        let right_velocity = vel_ref.get(1).unwrap();
        let left_acceleration = acc_ref.get(0).unwrap();
        let right_acceleration = acc_ref.get(1).unwrap();
        assert_eq!(left_position, &((-1.) * right_position));
        assert_eq!(left_velocity, &((-1.) * right_velocity));
        assert_eq!(left_acceleration, &((-1.) * right_acceleration));
        println!(
            "time = {}. position {:?} ||| velocity {:?} ||| acceleration {:?}",
            time_now,
            left_position.components(),
            left_velocity.components(),
            left_acceleration.components()
        );
    }
}

fn main() {
    let mut two = JobSetup::build()
        .delta_t(0.01)
        .boundaries(Region::new([10., 10.]))
        .state(TwoState::default())
        .potential(LennardJones::new(5.))
        .job();
    two.run(100);
    println!("Run 1 finished: {:?}", two);
}
