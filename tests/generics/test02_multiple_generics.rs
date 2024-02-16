use auto_delegate::{delegate, Delegate};

#[delegate]
trait Trait<'p, 'v, P, V> {
    fn p_ref(&self) -> &'p P;

    fn v_ref(&self) -> &'v V;
}

struct Child<'p, 'v, P, V> {
    p: &'p P,
    v: &'v V,
}

impl<'p, 'v, P, V> Child<'p, 'v, P, V> {
    pub fn new(p: &'p P, v: &'v V) -> Child<'p, 'v, P, V> {
        Self { p, v }
    }
}

impl<'p, 'v, P, V> Trait<'p, 'v, P, V> for Child<'p, 'v, P, V> {
    fn p_ref(&self) -> &'p P {
        self.p
    }

    fn v_ref(&self) -> &'v V {
        self.v
    }
}

#[derive(Delegate)]
struct Parent<'p, 'v, P, V> {
    #[to(Trait)]
    child: Child<'p, 'v, P, V>,
}

impl<'p, 'v, P, V> Parent<'p, 'v, P, V> {
    pub fn new(p: &'p P, v: &'v V) -> Parent<'p, 'v, P, V> {
        Self {
            child: Child::new(p, v),
        }
    }
}

fn main() {
    let p = String::new();
    let v = 32;

    let parent = Parent::new(&p, &v);

    assert_eq!(parent.p_ref(), "");
    assert_eq!(parent.v_ref(), &32);
}