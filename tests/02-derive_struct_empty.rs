use macros::{delegate, delegate_trait};


/// 構造体に対して、空のトレイトのトレイト名を指定して
/// 自動的に以上処理が定義される処理の追加

#[delegate_trait]
trait Empty {}


#[delegate(Empty)]
#[allow(dead_code)]
struct Object;

fn main() {}
