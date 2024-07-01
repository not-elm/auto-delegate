use proc_macro::TokenStream;

use ::syn::__private::TokenStream2;

use crate::delegate_trait::expand_delegate_trait;
use crate::derive::expand_derive_delegate;

mod derive;
mod delegate_trait;
mod ident;
mod delegatable;
mod syn;
mod trait_meta;
mod attribute;
mod async_trait;


/// The trait given this attribute are eligible for delegation.
///
/// ```rust
/// use auto_delegate::delegate;
///
/// #[delegate]
/// pub trait Readable{
///     fn read(&self) -> String;
/// }
/// ```
#[proc_macro_attribute]
pub fn delegate(attr: TokenStream, input: TokenStream) -> TokenStream {
    let output: proc_macro2::TokenStream = expand_delegate_trait(attr, input.clone());
    output.into()
}


/// Implement the specified trait and delegate its processing to the children.
///
///
/// ## Note
///
/// The trait to be delegated must be given a 'delegate'.
///
///
/// ## Example1:
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
/// ## Example2: Generics-type Child
///
/// It is possible to use generic type for member types
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
///
/// impl Calc for CalcAdd {
///     fn calc(&self, x1: usize, x2: usize) -> usize {
///         x1 + x2
///     }
/// }
///
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
/// ## Example3: Multiple traits and fields
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
/// ## Example4: Enum
///
///
/// ```rust
/// use auto_delegate::{delegate, Delegate};
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
/// #[derive(Default)]
/// struct CalcSub;
///
///
/// impl Calc for CalcSub {
///     fn calc(&self, x1: usize, x2: usize) -> usize {
///         x1 - x2
///     }
/// }
///
///
/// #[derive(Delegate)]
/// #[to(Calc)]
/// enum EnumCalc {
///     Add(CalcAdd),
///     Sub(CalcSub),
/// }
///
///
/// let c = EnumCalc::Add(CalcAdd::default());
/// assert_eq!(c.calc(3, 5), 8);
///
/// let c = EnumCalc::Sub(CalcSub::default());
/// assert_eq!(c.calc(3, 2), 1);
/// ```
///
#[proc_macro_derive(Delegate, attributes(to))]
pub fn derive_delegate(input: TokenStream) -> TokenStream {
    expand_derive_delegate(input).into()
}


pub(crate) fn intersperse(
    separator: TokenStream2,
    iter: impl Iterator<Item=TokenStream2>) -> Vec<TokenStream2> {
    let mut tokens = Vec::<TokenStream2>::new();
    for token in iter {
        tokens.push(token);
        tokens.push(separator.clone());
    }

    if !tokens.is_empty() {
        tokens.remove(tokens.len() - 1);
    }

    tokens
}