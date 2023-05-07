use macros::delegate_trait;

/// 構造体のメンバに自動移譲させるマクロを作成出来るようにします。
#[delegate_trait]
trait Adder {
    fn increment(&mut self) -> usize;
}


struct Child {
    num: usize,
}

impl Adder for Child {
    fn increment(&mut self) -> usize {
        self.num += 1;
        self.num
    }
}


struct Parent {
    child: Child,
}


fn main() {
    let mut parent = Parent {
        child: Child { num: 0 },
    };
    impl_delegate_adder!(Parent, child);
    assert_eq!(parent.increment(), 1);
    assert_eq!(parent.increment(), 2);
    assert_eq!(parent.increment(), 3);
}
