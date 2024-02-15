#![no_std]


pub use auto_delegate_impl::{delegate, Delegate};

#[doc(hidden)]
pub struct Delegates<A, B, C, D, E, F, G, H, I, J, K, L>(
    pub Option<A>,
    pub Option<B>,
    pub Option<C>,
    pub Option<D>,
    pub Option<E>,
    pub Option<F>,
    pub Option<G>,
    pub Option<H>,
    pub Option<I>,
    pub Option<J>,
    pub Option<K>,
    pub Option<L>,
);

macro_rules! expand_delegatable {
    ($($g: tt), *) => {
        #[doc(hidden)]
        pub trait Delegatable<$(const $g: char = ' ',)*> {
            type A;
            type B;
            type C;
            type D;
            type E;
            type F;
            type G;
            type H;
            type I;
            type J;
            type K;
            type L;

            fn delegate_by_owned(
                self
            ) -> Delegates<Self::A, Self::B, Self::C, Self::D, Self::E, Self::F, Self::G, Self::H, Self::I, Self::J, Self::K, Self::L>;

            fn delegate_by_ref(
                &self
            ) -> Delegates<&Self::A, &Self::B, &Self::C, &Self::D, &Self::E, &Self::F, &Self::G, &Self::H, &Self::I, &Self::J, &Self::K, &Self::L>;

            fn delegate_by_mut(
                &mut self
            ) -> Delegates<&mut Self::A, &mut Self::B, &mut Self::C, &mut Self::D, &mut Self::E, &mut Self::F, &mut Self::G, &mut Self::H, &mut Self::I, &mut Self::J, &mut Self::K, &mut Self::L>;
        }
    };
}

expand_delegatable!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC, AD
);
