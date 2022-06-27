#[allow(unused, dead_code)]

pub type Real = f32;

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let v = DVector::from(&[1., 0.22, 1e-6]);
        assert_eq!(&[1., 0.22, 1e-6], v.components());
    }
}
