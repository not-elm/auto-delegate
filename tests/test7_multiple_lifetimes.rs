use auto_delegate_macros::{delegate, Delegate};

#[delegate]
pub trait MultipleNames<'a, 'b> {
    fn name1(&self) -> &'a str;

    fn name2(&self) -> &'b str;
}


struct Child<'a, 'b> {
    name1: &'a str,

    name2: &'b str,
}


impl<'a, 'b> Child<'a, 'b> {
    pub fn new(name1: &'a str, name2: &'b str) -> Child<'a, 'b> {
        Self { name1, name2 }
    }
}


impl<'a, 'b> MultipleNames<'a, 'b> for Child<'a, 'b> {
    fn name1(&self) -> &'a str {
        &self.name1
    }

    fn name2(&self) -> &'b str {
        &self.name2
    }
}


#[derive(Delegate)]
struct Parent<'a, 'b> {
    #[to(MultipleNames)]
    child: Child<'a, 'b>,
}


impl<'a, 'b> Parent<'a, 'b> {
    pub fn new(name1: &'a str, name2: &'b str) -> Parent<'a, 'b> {
        Self {
            child: Child::new(name1, name2),
        }
    }
}

fn main() {
    let name1 = "Rust";
    let name2 = "Kotlin";

    // let parent = Parent::new(name1, name2);
    let parent = Child::new(name1, name2);

    assert_eq!(parent.name1(), "Rust");
    assert_eq!(parent.name2(), "Kotlin");
}
