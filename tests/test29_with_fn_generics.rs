#![allow(late_bound_lifetime_arguments)]

use std::any::type_name;
use std::fmt::Display;

use auto_delegate_impl::{delegate, Delegate};

#[delegate]
trait Trait {
    fn owned<S: Display>(self, s: S) -> S;


    fn reference<'a, S>(&self) where S: Display + 'a;

    fn mutable<'a, S>(&mut self) where S: Display + 'a;

    fn static_method<S: Display>(s: S) -> S;

    fn static_method2<S: Display>();
}


struct Child;

impl Trait for Child {
    fn owned<S: Display>(self, s: S) -> S {
        s
    }

    fn reference<'a, S>(&self) where S: Display + 'a {
        println!("{}", type_name::<S>());
    }

    fn mutable<'a, S>(&mut self) where S: Display + 'a {
        println!("{}", type_name::<S>());
    }

    fn static_method<S: Display>(s: S) -> S {
        s
    }

    fn static_method2<S: Display>() {
        println!("{}", type_name::<S>());
    }
}

#[derive(Delegate)]
struct Parent {
    #[to(Trait)]
    child: Child,
}

fn main() {
    assert_eq!(Parent::static_method("hello"), "hello");
    assert_eq!(Parent { child: Child }.owned("hello"), "hello");
    Parent { child: Child }.reference::<String>();
    Parent { child: Child }.mutable::<String>();
    Parent::static_method2::<String>();
}