use auto_delegate_macros::{delegate, Delegate};

#[delegate]
trait Buffer {
    fn buff(&mut self, array: [u32; 4]) -> [u32; 4];

    fn array<'b>(&mut self, array: &'b [u32]) -> &'b [u32];
}


#[derive(Default)]
struct Child;

impl Buffer for Child {
    fn buff(&mut self, array: [u32; 4]) -> [u32; 4] {
        array
    }

    fn array<'buff>(&mut self, array: &'buff [u32]) -> &'buff [u32] {
        array
    }
}


#[derive(Default, Delegate)]
struct Parent {
    #[to(Buffer)]
    child: Child,
}


fn main() {
    let mut parent = Parent::default();

    assert!(parent
        .buff([3; 4])
        .into_iter()
        .all(|e| e == 3));

    let array = [3; 4];

    assert!(parent
        .array(&array)
        .iter()
        .all(|b| *b == 3))
}
