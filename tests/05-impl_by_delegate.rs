use macros::{delegate_trait, Delegate};
//
// /// delegateによって構造体のメンバに処理を委譲させるようにする。
// /// 具体的には、レシーバ(なし、&self, &mut self, self)と引数の
// /// 有無の判断および、実装を行う。
// ///
// /// ただし、実装の中身に関してはまだ実装しない。
// #[delegate_trait]
// trait DummyNumber {
//     fn increment(&mut self, num: usize) -> usize;
// }
//
//
// struct ChildImpl;
//
// impl DummyNumber for ChildImpl {
//     fn increment(&mut self, num: usize) -> usize {
//         num + 1
//     }
// }
//
//
// #[derive(Delegate)]
// struct ParentImpl {
//     #[by(DummyNumber)]
//     child: ChildImpl,
// }
//
//
// struct Impl {}
//
// fn main() {
//     impl_delegate_dummynumber!(ParentImpl);
//     let mut parent = ParentImpl {
//         child: ChildImpl {},
//     };
//
//     assert_eq!(parent.increment(6), 7);
// }
