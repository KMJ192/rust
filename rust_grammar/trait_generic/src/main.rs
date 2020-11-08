trait Shape{
    fn area(&self) -> u32;
}

struct Rectangle{
    x : u32,
    y : u32,
}

struct Circle{
    radius : f64,
}

impl Shape for Rectangle{
    fn area(&self) -> u32{
        self.x * self.y
    }
}

impl Shape for Circle{
    fn area(&self) -> u32{
        (3.14 * self.radius * self.radius) as u32
    }
}

use std::ops;

#[derive(Debug, Clone, Copy)] //Debug, Clone, Copy가능
struct A;
struct B;

#[derive(Debug)]
struct AB;
#[derive(Debug)]
struct BA;

impl ops::Add(B) for A{
    type Output = AB;
    fn add(self, _rhs : B) -> AB{
        AB
    }
}
impl ops::Add<A> for B{
    type Output = BA;
    fn add(self, _rhs : A) -> BA{
        BA
    }
}

fn main() {
    let c = Circle{radius : 100.132};
    let r = Rectangle {x : 30, y : 20};
    println!("{}, {}", c.area(), r.area());

    let a = A(32);
    let b = B(12.13);
    let c = a;

    println!("{:?}", a);
}
