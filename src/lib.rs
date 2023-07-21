#![no_std]

pub use auto_delegate_impl::{delegate, Delegate};

macro_rules! expand_macro_maker {
    ($($g: tt), *) => {
        #[doc(hidden)]
        pub trait MacroMarker<$(const $g: char = ' ',)*> {
            type DelegateType: ?core::marker::Sized;

            #[doc(hidden)]
            fn delegate_by_ref(
                &self
            ) -> &Self::DelegateType;


            #[doc(hidden)]
            fn delegate_by_mut(
                &mut self
            ) -> &mut Self::DelegateType;
        }
    };
}

expand_macro_maker!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC, AD
);
