# Auto Delegate

## Description

## Usage

### Example: Delegate by child with the struct

```rust
use auto_delegate::{Delegate, delegate};

#[delegate]
trait Calc {
    fn calc(&self, x1: usize, x2: usize) -> usize;
}


struct CalcAdd;

impl Calc for CalcAdd {
    fn calc(&self, x1: usize, x2: usize) -> usize {
        x1 + x2
    }
}

#[derive(Delegate)]
struct Parent {
    #[to(Calc)]
    child: CalcAdd
}


fn main() {
    let parent = Parent { child: CalcAdd {} };
    assert_eq!(parent.calc(2, 3), 5);
}
```