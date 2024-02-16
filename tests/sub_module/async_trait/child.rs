use async_trait::async_trait;
use crate::sub_module::async_trait::email_readable::{EmailAddress, EmailReadable};

#[derive(Default)]
pub struct EmailReader{
    email: EmailAddress
}

#[async_trait]
impl EmailReadable for EmailReader{
    async fn read_email<'a>(&'a self) -> &'a EmailAddress {
        &self.email
    }
}


