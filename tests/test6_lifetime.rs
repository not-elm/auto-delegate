use auto_delegate_macros::{delegate, Delegate};

/// 複数のトレイトを実装したフィールドから、
/// それぞれの処理を委譲できるようにします。
#[delegate]
trait StringRef<'a> {
    fn str_ref(&'a self) -> &'a str;
}


struct Child<'a> {
    name: &'a str,
}


impl<'a> StringRef<'a> for Child<'a> {
    fn str_ref(&'a self) -> &'a str {
        self.name.as_ref()
    }
}


impl<'a> Child<'a> {
    pub fn new(name: &'a str) -> Self {
        Self { name }
    }
}

#[derive(Delegate)]
struct Parent<'a> {
    #[to(StringRef)]
    child: Child<'a>,
}


impl<'a> Parent<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            child: Child::new(name),
        }
    }
}

fn main() {
    let name = "Rust";

    let parent = Parent::new(name);

    assert_eq!(parent.str_ref(), name);
}
