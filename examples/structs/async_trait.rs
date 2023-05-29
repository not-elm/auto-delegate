use async_trait::async_trait;

use auto_delegate::{delegate, Delegate};

pub struct EmailAddress(String);

impl EmailAddress {
    #[allow(unused)]
    pub fn raw(&self) -> &str {
        self.0.as_str()
    }
}


impl Default for EmailAddress {
    fn default() -> Self {
        Self(String::from("rust@gmail.com"))
    }
}


#[async_trait]
#[delegate]
pub trait EmailReadable {
    async fn read_email<'a>(&'a self) -> &'a EmailAddress;
}


#[derive(Default)]
pub struct EmailReader {
    email: EmailAddress,
}


#[async_trait]
impl EmailReadable for EmailReader {
    async fn read_email<'a>(&'a self) -> &'a EmailAddress {
        &self.email
    }
}


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