use std::{marker::PhantomData, ops::Add};

use auto_delegate::{delegate, Delegate};

#[delegate]
trait Addr<X, Y: Add<Output=X>>
    where
        X: Add<Y, Output=X>,
{
    fn add(&mut self, x: X, y: Y) -> X;
}

struct Child<X, Y: Add<Output=X>>
    where
        X: Add<Y, Output=X>,
{
    _maker: PhantomData<X>,
    _maker2: PhantomData<Y>,
}

impl<X, Y: Add<Output=X>> Addr<X, Y> for Child<X, Y>
    where
        X: Add<Y, Output=X>,
{
    fn add(&mut self, x: X, y: Y) -> X {
        x + y
    }
}

#[derive(Delegate)]
struct Parent<X, Y: Add<Output=X>>
    where
        X: Add<Y, Output=X>,
{
    #[to(Addr)]
    child: Child<X, Y>,
}

fn main() {
    let mut parent = Parent {
        child: Child {
            _maker: PhantomData,
            _maker2: PhantomData,
        },
    };

    assert_eq!(parent.add(2, 3), 5);
}