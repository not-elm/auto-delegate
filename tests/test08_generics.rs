use auto_delegate::{delegate, Delegate};

#[delegate]
trait Accessible<T> {
    fn read(&self) -> &T;

    fn read_mut(&mut self) -> &mut T;

    fn write(&mut self, value: T);
}


struct Child<T> {
    value: T,
}


impl<T: Default> Default for Child<T> {
    fn default() -> Self {
        Self {
            value: T::default(),
        }
    }
}


impl<T> Accessible<T> for Child<T> {
    fn read(&self) -> &T {
        &self.value
    }

    fn read_mut(&mut self) -> &mut T {
        &mut self.value
    }

    fn write(&mut self, value: T) {
        self.value = value;
    }
}


#[derive(Delegate)]
struct Parent<T> {
    #[to(Accessible)]
    child: Child<T>,
}


impl<T: Default> Default for Parent<T> {
    fn default() -> Parent<T> {
        Self {
            child: Child::default(),
        }
    }
}


fn main() {
    let mut parent = Parent::<String>::default();
    assert_eq!(parent.read(), "");
    parent.write(String::from("Hello"));
    assert_eq!(parent.read_mut(), "Hello");
}
