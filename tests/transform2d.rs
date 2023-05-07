use macros::delegate_trait;

#[delegate_trait]
pub trait Transformable2D{
    fn move_to(
        &mut self,
        x: usize,
        y: usize);

    fn pos(&self) -> (usize, usize);
}


