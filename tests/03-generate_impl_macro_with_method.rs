use macros::delegate_trait;

/// 1つのメソッドを持つトレイトに対して"delegate_trait"を付与した場合に
/// 正確な名前でメソッドが宣言されること。
///
/// ただし、委譲の引数(レシーバも含む)及び中身自体は03では定義せず、
/// 1を返すだけの処理とする。
#[delegate_trait]
trait DummyNumber {
    fn num() -> usize;
    fn num2() -> usize;
    fn num1000() -> usize;
}


struct Impl {}

fn main() {
    impl_delegate_dummynumber!(Impl);
    assert_eq!(Impl::num(), 1);
    assert_eq!(Impl::num2(), 1);
    assert_eq!(Impl::num1000(), 1);
}
