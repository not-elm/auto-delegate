use async_trait::async_trait;

use auto_delegate::{Delegate, delegate};

#[async_trait]
#[delegate]
pub trait Calc {
    async fn calc(&self, x1: usize, x2: usize) -> usize;
}


#[derive(Default)]
pub struct Child;

#[async_trait]
impl Calc for Child {
    async fn calc(&self, x1: usize, x2: usize) -> usize {
        x1 + x2
    }
}


#[derive(Delegate, Default)]
#[to(Calc)]
pub struct Parent(Child);


#[tokio::main]
async fn main() {
    let parent = Parent::default();

    assert_eq!(parent.calc(3, 2).await, 5);
}



