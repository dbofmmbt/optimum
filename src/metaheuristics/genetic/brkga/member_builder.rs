use super::BrkgaMember;

use rand::Rng;

use super::super::Decoder;

#[non_exhaustive]
pub struct Ctx<'a, D, R> {
    pub decoder: &'a D,
    pub rng: &'a mut R,
    pub member_size: usize,
}

pub trait MemberBuilder<D: Decoder, R: Rng> {
    fn build(&mut self, ctx: &mut Ctx<'_, D, R>, member_number: usize) -> BrkgaMember<D>;
}

impl<D: Decoder, R: Rng, T: FnMut(&mut Ctx<'_, D, R>, usize) -> BrkgaMember<D>> MemberBuilder<D, R>
    for T
{
    fn build(&mut self, ctx: &mut Ctx<'_, D, R>, member_number: usize) -> BrkgaMember<D> {
        self(ctx, member_number)
    }
}

/// Generates random members by using the BRKGA's RNG to generate floats.
pub struct RandomMemberBuilder;

impl<R: Rng, D: Decoder> MemberBuilder<D, R> for RandomMemberBuilder {
    fn build(&mut self, ctx: &mut Ctx<'_, D, R>, _: usize) -> BrkgaMember<D> {
        let keys = {
            let mut k = vec![0.0; ctx.member_size].into_boxed_slice();
            ctx.rng.fill(k.as_mut());
            k
        };

        let value = ctx.decoder.decode_value(&keys);
        BrkgaMember::<D> { keys, value }
    }
}
