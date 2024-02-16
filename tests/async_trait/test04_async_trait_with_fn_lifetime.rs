#![allow(late_bound_lifetime_arguments)]

use auto_delegate_impl::{Delegate, delegate};

#[delegate]
trait ReadEmailAddress {
    async fn read_email_address<'a>(&'a self) -> &'a str;
}

pub struct EmailAddress<'a>(&'a str);

impl<'a> ReadEmailAddress for EmailAddress<'a> {
    async fn read_email_address<'b>(&'b self) -> &'b str {
        self.0
    }
}


#[derive(Delegate)]
struct Parent<'a> {
    #[to(ReadEmailAddress)]
    address: EmailAddress<'a>,
}

#[tokio::main]
async fn main() {
    let parent = Parent {
        address: EmailAddress("example@gmail.com")
    };

    assert_eq!(
        parent.read_email_address().await,
        "example@gmail.com"
    );
}
