#![no_std]


pub use auto_delegate_impl::{delegate, Delegate};

#[doc(hidden)]
pub struct Marker<A, B, C, D, E, F, G, H, I, J, K, L>(
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

macro_rules! expand_macro_maker {
    ($($g: tt), *) => {
        #[doc(hidden)]
        pub trait MacroMarker<$(const $g: char = ' ',)*> {
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
            ) -> Marker<Self::A, Self::B, Self::C, Self::D, Self::E, Self::F, Self::G, Self::H, Self::I, Self::J, Self::K, Self::L>;

            fn delegate_by_ref(
                &self
            ) -> Marker<&Self::A, &Self::B, &Self::C, &Self::D, &Self::E, &Self::F, &Self::G, &Self::H, &Self::I, &Self::J, &Self::K, &Self::L>;

            fn delegate_by_mut(
                &mut self
            ) -> Marker<&mut Self::A, &mut Self::B, &mut Self::C, &mut Self::D, &mut Self::E, &mut Self::F, &mut Self::G, &mut Self::H, &mut Self::I, &mut Self::J, &mut Self::K, &mut Self::L>;
        }
    };
}

expand_macro_maker!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC, AD
);
