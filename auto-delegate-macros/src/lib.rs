#![feature(iter_intersperse)]
#![feature(core_intrinsics)]
#![feature(proc_macro_span)]
#![feature(iterator_try_collect)]


use proc_macro::TokenStream;

use crate::delegate_trait::expand_delegate_trait;
use crate::derive_delegate::expand_derive_delegate;

mod derive_delegate;
mod delegate_trait;
mod ident;
mod macro_marker;
mod span;
mod syn;
mod trait_item;
mod attribute;


///
///
/// ## Example1: Delegate To Child
///
/// ```rust
/// use auto_delegate::*;
///
/// #[delegate]
/// trait Calc {
///     fn calc(&self, x1: usize, x2: usize) -> usize;
/// }
///
/// #[derive(Default)]
/// struct CalcAdd;
///
/// impl Calc for CalcAdd {
///     fn calc(&self, x1: usize, x2: usize) -> usize {
///         x1 + x2
///     }
/// }
///
/// #[derive(Delegate, Default)]
/// struct Parent {
///     #[to(Calc)]
///     child: CalcAdd
/// }
///
/// let parent =  Parent::default();
///
/// assert_eq!(parent.calc(3, 2), 5);
/// ```
///
///
/// ## Example2: Delegate To Child
///
/// It is possible to use generic param for member types
///
/// ```rust
/// use auto_delegate::*;
///
/// #[delegate]
/// trait Calc {
///     fn calc(&self, x1: usize, x2: usize) -> usize;
/// }
///
/// #[derive(Default)]
/// struct CalcAdd;
///
/// impl Calc for CalcAdd {
///     fn calc(&self, x1: usize, x2: usize) -> usize {
///         x1 + x2
///     }
/// }
///
/// #[derive(Default, Delegate)]
/// struct Parent<T: Default + Calc>{
///     #[to(Calc)]
///     child: T
/// }
///
///
/// let parent = Parent::<CalcAdd>::default();
///
/// assert_eq!(parent.calc(3, 2), 5);
/// ```
///
/// ## Example3: Delegate To Child
///
/// It is possible to use generic param for member types
///
/// ```rust
/// use auto_delegate::*;
///
/// #[delegate]
/// trait Calc {
///     fn calc(&self, x1: usize, x2: usize) -> usize;
/// }
///
///
/// #[derive(Default)]
/// struct CalcAdd;
///
///
/// impl Calc for CalcAdd {
///     fn calc(&self, x1: usize, x2: usize) -> usize {
///         x1 + x2
///     }
/// }
///
///
/// #[delegate]
/// trait Movable{
///     fn move_to(&mut self, pos: (usize, usize));
///
///     fn pos(&self) -> (usize, usize);
/// }
///
///
/// #[delegate]
/// trait Resizable{
///     fn resize(&mut self, width: usize, height: usize);
///
///     fn size(&self) -> (usize, usize);
/// }
///
/// #[derive(Default)]
/// struct Transform2D{
///     pos: (usize, usize),
///     width: usize,
///     height: usize
/// }
///
///
/// impl Movable for Transform2D {
///     fn move_to(&mut self, pos: (usize, usize)) {
///         self.pos = pos;
///     }
///
///     fn pos(&self) -> (usize, usize) {
///         self.pos
///     }
/// }
///
///
/// impl Resizable for Transform2D{
///     fn resize(&mut self, width: usize, height: usize) {
///         self.width = width;
///         self.height = height;
///     }
///
///     fn size(&self) -> (usize, usize){
///         (self.width, self.height)
///     }
/// }
///
/// #[derive(Default, Delegate)]
/// struct Parent<T: Default + Calc>{
///     #[to(Movable, Resizable)]
///     transform: Transform2D,
///
///     #[to(Calc)]
///     calculator: T
/// }
///
///
/// let mut parent = Parent::<CalcAdd>::default();
///
/// assert_eq!(parent.calc(3, 2), 5);
///
/// parent.move_to((10, 11));
/// assert_eq!(parent.pos(), (10, 11));
///
/// parent.resize(100, 120);
/// assert_eq!(parent.size(), (100, 120));
/// ```
///
#[proc_macro_attribute]
pub fn delegate(attr: TokenStream, input: TokenStream) -> TokenStream {
    let output: proc_macro2::TokenStream = expand_delegate_trait(attr, input.clone());
    expand_join(input, output)
}


#[proc_macro_derive(Delegate, attributes(to))]
pub fn derive_delegate(input: TokenStream) -> TokenStream {
    expand_derive_delegate(input).into()
}


fn expand_join(input: TokenStream, output: proc_macro2::TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    let expand = quote::quote! {
        #input
        #output
    };

    expand.into()
}


