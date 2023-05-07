#[delegatable]
trait Transformable2D{
    fn move_to(
        &mut self,
        x: usize,
        y: usize);

    fn pos(&self) -> (usize, usize);
}




struct Transform2D{
    pub pos_x: usize,
    pub pos_y: usize
}


impl Default for Transform2D{
    fn default() -> Self {
        Self{
            pos_x: 0,
            pos_y: 0
        }
    }
}


impl Transformable2D for Transform2D{
    fn move_to(&mut self, x: usize, y: usize) {
        self.pos_x = x;
        self.pos_y = y;
    }


    fn pos(&self) -> (usize, usize){
        (self.pos_x, self.pos_y)
    }
}




#[Delegate(Transformable2D), Default]
struct Object{
    transform: Transform2D
}



fn main(){
    let o = Object::default();
    o.move_to(3, 5);
    let (x, y) = o.pos();
    assert_eq!(x, 3);
    assert_eq!(y, 5);
}