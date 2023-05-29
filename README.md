# Auto Delegate

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

#### Example2: Generics-type Child

It is possible to use generic type for member types

```rust
#[derive(Default, Delegate)]
struct Parent<T: Default + Calc> {
    #[to(Calc)]
    child: T,
}


fn main() {
    let parent = Parent::<CalcAdd>::default();

    assert_eq!(parent.calc(3, 2), 5);
}
```

#### Example3: Multiple traits and fields

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

#### Example4 New type

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


#[derive(Default, Delegate)]
#[to(Calc)]
struct Parent<T>(T) where T: Calc + Default;

fn main() {
    let parent = Parent::<CalcAdd>::default();
    assert_eq!(parent.calc(2, 3), 5);
}
```

#### Example5 Async trait

```rust
use async_trait::async_trait;

use auto_delegate::{delegate, Delegate};

pub struct EmailAddress(String);

impl EmailAddress {
    #[allow(unused)]
    pub fn raw(&self) -> &str {
        self.0.as_str()
    }
}


impl Default for EmailAddress {
    fn default() -> Self {
        Self(String::from("rust@gmail.com"))
    }
}


#[async_trait]
#[delegate]
pub trait EmailReadable {
    async fn read_email<'a>(&'a self) -> &'a EmailAddress;
}


#[derive(Default)]
pub struct EmailReader {
    email: EmailAddress,
}


#[async_trait]
impl EmailReadable for EmailReader {
    async fn read_email<'a>(&'a self) -> &'a EmailAddress {
        &self.email
    }
}


#[derive(Default, Delegate)]
struct Parent {
    #[to(EmailReadable)]
    child: EmailReader,
}


#[tokio::main]
async fn main() {
    let parent = Parent::default();

    assert_eq!(parent.read_email().await.raw(), "rust@gmail.com");
}
```

#### Example6 With Super traits

```rust
use auto_delegate::{delegate, Delegate};

#[delegate]
trait Calc {
    fn calc(&self, x1: usize, x2: usize) -> usize;
}


#[delegate]
trait Increment: Calc {
    fn increment(&mut self) -> usize;
}


#[derive(Default)]
struct Child {
    num: usize,
}


impl Increment for Child {
    fn increment(&mut self) -> usize {
        self.num += 1;
        self.num
    }
}


impl Calc for Child {
    fn calc(&self, x1: usize, x2: usize) -> usize {
        x1 + x2
    }
}


#[derive(Default, Delegate)]
struct Parent {
    #[to(Calc, Increment)]
    child: Child,
}


fn main() {
    let mut parent = Parent::default();
    assert_eq!(parent.increment(), 1);
    assert_eq!(parent.increment(), 2);
    assert_eq!(parent.calc(3, 2), 5);
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

