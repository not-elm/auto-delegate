use auto_delegate_impl::{delegate, Delegate};

/// 複数のメソッド、返り値がないメソッドを持つトレイトを委譲できるようにします。
#[delegate]
trait StringRef {
    fn str_ref(&self) -> &str;

    fn str_mut(&mut self) -> &mut str;

    fn str_change(&mut self, new_val: &str) -> &str;
}

struct Child {
    str: String,
}

impl Default for Child {
    fn default() -> Self {
        Self {
            str: String::from("Hello World!"),
        }
    }
}

impl StringRef for Child {
    fn str_ref(&self) -> &str {
        self.str.as_str()
    }

    fn str_mut(&mut self) -> &mut str {
        self.str.as_mut_str()
    }

    fn str_change(&mut self, new_val: &str) -> &str {
        self.str = String::from(new_val);
        self.str_ref()
    }
}

#[derive(Delegate, Default)]
struct Parent {
    #[to(StringRef)]
    child: Child,
}

fn main() {
    let mut parent = Parent::default();

    assert_eq!(parent.str_ref(), "Hello World!");
    assert_eq!(parent.str_change("New Val"), "New Val");
    assert_eq!(parent.str_mut(), "New Val");
}
