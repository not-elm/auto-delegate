use macros::{delegate_trait, Delegate};

/// メソッドの返り値が参照型になるメソッドに対応させます。
#[delegate_trait]
trait Transformable {
    fn position_x_ref(&self) -> &usize;
    fn position_x_mut(&mut self) -> &mut usize;
    fn display_str(&self) -> &str;
}


struct Transform {
    x: usize,
    str: String,
}


impl Transformable for Transform {
    fn position_x_ref(&self) -> &usize {
        &self.x
    }

    fn position_x_mut(&mut self) -> &mut usize {
        &mut self.x
    }

    fn display_str(&self) -> &str {
        self.str.as_str()
    }
}


#[derive(Delegate)]
struct Parent {
    #[by(Transformable)]
    transform: Transform,
}


fn main() {
    let mut parent = Parent {
        transform: Transform {
            x: 0,
            str: String::from("hello"),
        },
    };


    assert_eq!(*parent.position_x_ref(), 0);
    *parent.position_x_mut() = 3;
    assert_eq!(*parent.position_x_ref(), 3);

    assert_eq!(parent.display_str(), "hello");
}
