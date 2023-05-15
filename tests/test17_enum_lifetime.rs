// use std::ops::{Add, AddAssign, Sub};

// use auto_delegate_macros::{delegate, Delegate};

// use crate::common::increment::Increment;

// mod common;

// #[delegate]
// trait Calc<T> {
//     fn calc(&mut self, x1: T, x2: T) -> T;
// }

// struct CalcAdd<T> {
//     num: T,
// }


// impl<T: Add> Calc<T> for CalcAdd<T> {
//     fn calc(&mut self, x1: usize, x2: usize) -> usize {
//         x1 + x2
//     }
// }


// impl Increment for CalcAdd<usize> {
//     fn increment(&mut self) -> usize {
//         self.num += 1;
//         self.num
//     }
// }


// struct CalcSub<T> {
//     num: T,
// }


// impl<T: Sub> Calc<T> for CalcSub<T> {
//     fn calc(&mut self, x1: T, x2: T) -> T {
//         x1 - x2
//     }
// }


// impl Increment for CalcSub<usize> {
//     fn increment(&mut self) -> usize {
//         self.num -= 1;
//         self.num
//     }
// }


// #[derive(Delegate)]
// #[to(Calc, Increment)]
// enum EnumCalc<T> {
//     Add(CalcAdd<T>),
//     Sub(CalcSub<T>),
// }


// fn main() {
//     let mut c = EnumCalc::Add(CalcAdd { num: 0 });
//     assert_eq!(*c.calc(3, 5), 8);
//     assert_eq!(c.increment(), 9);

//     let mut c = EnumCalc::Sub(CalcSub { num: 1 });
//     assert_eq!(*c.calc(3, 2), 1);
//     assert_eq!(c.increment(), 0);
// }
