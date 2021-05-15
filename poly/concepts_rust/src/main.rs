pub trait IDemo {
    fn some_operation(&self, a: u32, b: u32) -> u32;
}

pub struct ChildA;
impl IDemo for ChildA {
    fn some_operation(&self, a: u32, b: u32) -> u32 {
        1
    }
}

pub struct ChildB;
impl IDemo for ChildB {
    fn some_operation(&self, a: u32, b: u32) -> u32 {
        2
    }
}

fn operation(demo: &dyn IDemo) -> u32 {
    demo.some_operation(1,2)
}

pub struct Container<'a> {
    child: &'a dyn IDemo
}

impl<'a> Container<'a> {
    pub fn new(value: &'a dyn IDemo) -> Container {
        Container {
            child: value
        }
    }
}

fn main() {
    let a = ChildA;
    let b = ChildB;

    let value_a = operation(&a);
    let value_b = operation(&b);

    let c = Container::new(&a);
    c.child.some_operation(1,2);
    println!("a: {}, b: {}", value_a, value_b);
}
