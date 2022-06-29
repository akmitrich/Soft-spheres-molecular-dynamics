use std::ops::{AddAssign, Add};

#[allow(unused, dead_code)]

pub type Real = f32;

#[derive(Debug, Clone, PartialEq)]
pub struct DVector<const D: usize> {
    components: [Real; D],
}

impl<const D: usize> DVector<D> {
    pub fn components(&self) -> &[Real; D] {
        &self.components
    }
}

impl<const D: usize> Default for DVector<D> {
    fn default() -> Self {
        Self {
            components: [0 as Real; D],
        }
    }
}

impl<const D: usize> From<&[Real; D]> for DVector<D> {
    fn from(data: &[Real; D]) -> Self {
        Self::from(*data)
    }
}

impl<const D: usize> From<[Real; D]> for DVector<D> {
    fn from(components: [Real; D]) -> Self {
        Self { components }
    }
}

impl<const D: usize> AddAssign for DVector<D> {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..D {
            self.components[i] += rhs.components()[i];
        }
    }
}

impl<const D: usize> AddAssign<&Self> for DVector<D> {
    fn add_assign(&mut self, rhs: &Self) {
        for i in 0..D {
            self.components[i] += rhs.components()[i];
        }
    }
}

impl<const D: usize> Add for DVector<D> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut sum = self.clone();
        sum += rhs;
        sum
    }
}

impl<const D: usize> Add for &DVector<D> {
    type Output = DVector<D>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut sum: DVector<D> = self.clone();
        sum += rhs;
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let v = DVector::from(&[1., 0.22, 1e-6]);
        assert_eq!(&[1., 0.22, 1e-6], v.components());
    }

    #[test]
    fn add_assign() {
        let mut v = DVector::from([1., 2., 3.]);
        v += DVector::from([1., 0., 1.]);
        assert_eq!(&[2., 2., 4.], v.components());
        v += DVector::from([0., 0., -2.]);
        assert_eq!(&[2.; 3], v.components());
        v += v.clone(); // How to make v += &v?
        assert_eq!(&[4.; 3], v.components());
        let b = DVector::from([0., 1.5, 6.3]);
        v += &b;
        assert_eq!(&[4., 5.5, 10.3], v.components());    
    }

    #[test]
    fn add() {
        let a = DVector::from([1., 2., 3.]);
        let b = DVector::from([4., 5., 6.]);
        assert_eq!(&[2., 4., 6.], (&a + &a).components());
        assert_eq!(&[5., 7., 9.], (&a + &b).components());
        assert_eq!(&[5., 7., 9.], (&b + &a).components());
        assert_eq!(&[5., 7., 9.], (a + b).components());
    }
}
