use async_trait::async_trait;
use auto_delegate_impl::delegate;

#[delegate]
#[async_trait]
trait Increment {
    async fn increment(&mut self) -> usize;
}

#[derive(Default)]
struct A(usize);

#[async_trait]
impl Increment for A {
    async fn increment(&mut self) -> usize {
        self.0 += 1;
        self.0
    }
}

#[allow(unused)]
#[tokio::main]
async fn main() {
    assert_eq!(A::default().increment().await, 1);
}
