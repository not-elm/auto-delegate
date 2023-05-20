use auto_delegate_macros::Delegate;
use common::async_trait::email_readable::EmailReadable;

use crate::common::async_trait::child::EmailReader;

mod common;

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