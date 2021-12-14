use super::Key;
use crate::core::Problem;

pub trait Decoder {
    type P: Problem;

    fn decode(&self, member: &[Key]) -> <Self::P as Problem>::Value;
}
