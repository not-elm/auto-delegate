use async_trait::async_trait;
use auto_delegate_impl::{delegate, Delegate};

#[delegate]
#[async_trait]
trait AsyncTrait {
    async fn parse_usize(&mut self, num_str: &str) -> usize;
    
    async fn cache(&self) -> usize;
}

#[derive(Default)]
struct Child{
    cache: usize
}

#[async_trait]
impl AsyncTrait for Child{
    async fn parse_usize(&mut self, num_str: &str) -> usize {
        let num: usize = num_str.parse().unwrap();
        self.cache = num;
        num
    }

    async fn cache(&self) -> usize {
        self.cache
    }
}

#[derive(Default, Delegate)]
struct Parent {
    #[to(AsyncTrait, RegularAsync)]
    child: Child,
}


#[tokio::main]
async fn main() {
    let mut parent = Parent::default();

    assert_eq!(parent.parse_usize("2").await, 2);
    assert_eq!(parent.cache().await, 2);
}