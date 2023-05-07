use macros::delegate_trait;

#[delegate_trait]
trait Empty {}

#[allow(dead_code)]
struct Object;

fn main() {
    impl_delegate_empty!(Object);
}
