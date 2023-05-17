# Auto Delegate

## Description

Auto delegate allows you that automatic impl of traits and delegate their handling to children.

## Usage

### Example: Delegate by children with the struct

```rust
use auto_delegate::{Delegate, delegate};

#[delegate]
trait Calc {
    fn calc(&self, x1: usize, x2: usize) -> usize;
}

#[derive(Default)]
struct CalcAdd;

impl Calc for CalcAdd {
    fn calc(&self, x1: usize, x2: usize) -> usize {
        x1 + x2
    }
}

#[derive(Delegate, Default)]
struct Parent {
    #[to(Calc)]
    child: CalcAdd
}


fn main() {
    let parent = Parent::default();

    assert_eq!(parent.calc(2, 3), 5);
}
```

### Example: Delegate by children with the Enum

```rust
use auto_delegate::{delegate, Delegate};

#[delegate]
trait Calc {
    fn calc(&self, x1: usize, x2: usize) -> usize;
}

#[derive(Default)]
struct CalcAdd;

impl Calc for CalcAdd {
    fn calc(&self, x1: usize, x2: usize) -> usize {
        x1 + x2
    }
}

#[derive(Default)]
struct CalcSub;

impl Calc for CalcSub {
    fn calc(&mut self, x1: usize, x2: usize) -> usize {
        x1 - x2
    }
}


#[derive(Delegate)]
#[to(Calc)]
enum EnumCalc {
    Add(CalcAdd),
    Sub(CalcSub),
}


fn main() {
    let c = EnumCalc::Add(CalcAdd::default());
    assert_eq!(c.calc(3, 5), 8);


    let c = EnumCalc::Sub(CalcSub::default());
    assert_eq!(c.calc(3, 2), 1);
}
```

## Benchmarks

### Struct

```
Running benches/cmp_handwritten_struct.rs (target/release/deps/cmp_handwritten_struct-109bf81facbe75f1)
generate_vs_handwritten/generate
                        time:   [1.9793 ns 1.9840 ns 1.9907 ns]
                        change: [+0.2554% +0.6270% +1.0228%] (p = 0.00 < 0.05)
                        Change within noise threshold.
                        
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe
generate_vs_handwritten/handwritten
                        time:   [1.9737 ns 1.9799 ns 1.9874 ns]
                        change: [-0.2267% +0.1596% +0.5187%] (p = 0.42 > 0.05)
                        No change in performance detected.

```