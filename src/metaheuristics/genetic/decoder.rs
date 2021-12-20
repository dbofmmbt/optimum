use crate::core::Problem;

pub(crate) type Value<D> = <<D as Decoder>::P as Problem>::Value;

/// [Decoder] defines the behavior needed for Random-Key based algorithms to [decode][Decoder::decode] the member
/// of the population the discover its [Value][Problem::Value] for the target [Problem].
pub trait Decoder {
    /// The problem being decoded.
    type P: Problem;

    /// A mapping between a slice of [RandomKey]s and a value for [Self::P].
    ///
    /// You'll usually convert `member` into a solution for the problem and then
    /// apply the solution into the problem's objective function.
    fn decode(&self, member: &[RandomKey]) -> <Self::P as Problem>::Value;
}

// NOTE should this be another type?
/// A float number that belongs to [0, 1)
pub type RandomKey = f64;
