// use macros::{delegate_trait, Delegate};

// /// 複数のフィールド、複数のトレイトを委譲できるようにします。
// #[delegate_trait]
// trait Transformable {
//     fn move_to_x(&mut self, x: usize);
//     fn move_to_y(&mut self, y: usize);
// }

// #[delegate_trait]
// trait Resizable {
//     fn resize(&mut self, width: usize, height: usize);
// }


// #[delegate_trait]
// trait Number {
//     fn num(&self) -> i32;
// }


// #[delegate_trait]
// trait NumberString {
//     fn num_string(&self) -> String;
// }


// struct Transform {
//     x: usize,
//     y: usize,
//     width: usize,
//     height: usize,
// }


// impl Transform {
//     pub const fn new() -> Self {
//         Self {
//             x: 0,
//             y: 0,
//             width: 0,
//             height: 0,
//         }
//     }
// }


// impl Transformable for Transform {
//     fn move_to_x(&mut self, x: usize) {
//         self.x = x;
//     }

//     fn move_to_y(&mut self, y: usize) {
//         self.y = y;
//     }
// }


// impl Resizable for Transform {
//     fn resize(&mut self, width: usize, height: usize) {
//         self.width = width;
//         self.height = height;
//     }
// }


// struct Number3;

// impl Number for Number3 {
//     fn num(&self) -> i32 {
//         3
//     }
// }


// impl NumberString for Number3 {
//     fn num_string(&self) -> String {
//         String::from("3")
//     }
// }


// #[derive(Delegate)]
// struct Parent {
//     #[by(Transformable, Resizable)]
//     child: Transform,
//     #[by(Number, NumberString)]
//     number3: Number3,
// }


// fn main() {
//     let mut parent = Parent {
//         child: Transform::new(),
//         number3: Number3 {},
//     };

//     parent.move_to_x(3);
//     assert_eq!(parent.child.x, 3);

//     parent.move_to_y(5);
//     assert_eq!(parent.child.y, 5);


//     parent.resize(100, 300);
//     assert_eq!(parent.child.width, 100);
//     assert_eq!(parent.child.height, 300);


//     assert_eq!(parent.num(), 3);
//     assert_eq!(parent.num_string(), "3");
// }
