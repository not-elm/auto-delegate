use macros::delegate_trait;

/// "delegate_trait"によって生成されたメソッドが引数に対応すること。
/// 具体的には、レシーバ(なし、&self, &mut self, self)と引数の
/// 有無の判断および、実装を行う。
///
/// ただし、実装の中身に関してはまだ実装しない。
#[delegate_trait]
trait DummyNumber {
    fn increment(num: usize) -> usize;
    fn increment_ref(&self, num: usize) -> usize;
    fn increment_mut(&mut self, num: usize) -> usize;
    fn increment_own(self, num: usize) -> usize;
}


struct Impl {}

fn main() {
    impl_delegate_dummynumber!(Impl);

    assert_eq!(Impl::increment(1), 1);

    let mut i = Impl {};
    assert_eq!(i.increment_ref(1), 1);
    assert_eq!(i.increment_mut(1), 1);
    assert_eq!(i.increment_own(1), 1);
}
