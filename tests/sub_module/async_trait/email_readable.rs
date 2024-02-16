use async_trait::async_trait;

use auto_delegate_impl::delegate;

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

