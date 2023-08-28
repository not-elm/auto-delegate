# auto-delegate

## Supports `no_std`

This Library is Created without std crates.

## Description

Auto delegate allows you that automatic impl of traits and delegate their handling to children.

## Usage

### Struct

Give 'delegate' attribute to the trait which to be delegated,
and give 'Delegate' to the structure requesting delegation

#### Example1

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

#### Example2: Multiple traits and fields

```rust
use auto_delegate::*;

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


#[delegate]
trait Movable {
    fn move_to(&mut self, pos: (usize, usize));

    fn pos(&self) -> (usize, usize);
}


#[delegate]
trait Resizable {
    fn resize(&mut self, width: usize, height: usize);

    fn size(&self) -> (usize, usize);
}


#[derive(Default)]
struct Transform2D {
    pos: (usize, usize),
    width: usize,
    height: usize
}


impl Movable for Transform2D {
    fn move_to(&mut self, pos: (usize, usize)) {
        self.pos = pos;
    }

    fn pos(&self) -> (usize, usize) {
        self.pos
    }
}


impl Resizable for Transform2D {
    fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
    }

    fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }
}


#[derive(Default, Delegate)]
struct Parent<T: Default + Calc> {
    #[to(Movable, Resizable)]
    transform: Transform2D,

    #[to(Calc)]
    calculator: T
}


fn main() {
    let mut parent = Parent::<CalcAdd>::default();

    assert_eq!(parent.calc(3, 2), 5);

    parent.move_to((10, 11));
    assert_eq!(parent.pos(), (10, 11));

    parent.resize(100, 120);
    assert_eq!(parent.size(), (100, 120));
}
```

### Enum

Like Struct, Enum can be delegated using Delegate.

#### Example

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
    fn calc(&self, x1: usize, x2: usize) -> usize {
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

## Todo

- [x] support self-receiver(v0.0.8)
- [ ] support method without receiver