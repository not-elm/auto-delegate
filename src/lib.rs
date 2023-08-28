#![no_std]

extern crate alloc;


pub use auto_delegate_impl::{delegate, Delegate};

pub struct Marker<A, B = A, C = A, D = A>(
    pub Option<A>,
    pub Option<B>,
    pub Option<C>,
    pub Option<D>,
);

macro_rules! expand_macro_maker {
    ($($g: tt), *) => {
        #[doc(hidden)]
        pub trait MacroMarker<$(const $g: char = ' ',)*> {
            type DelegateType;
            type B;
            type C;
            type D;

            fn delegate_by_owned(
                self
            ) -> Marker<Self::DelegateType, Self::B, Self::C, Self::D>;

            fn delegate_by_ref(
                &self
            ) -> &Self::DelegateType;

            fn delegate_by_mut(
                &mut self
            ) -> &mut Self::DelegateType;
        }
    };
}

expand_macro_maker!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC, AD
);
