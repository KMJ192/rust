use std::fmt;
use std::ops::Mul;

#[derive(Debug)]
struct Square<T>{
    x : T,
}

fn p<T : fmt::Debug>(x : T){
    println!("{:?}", x);
}

struct A<T>{
    x :T,
}

impl <T> A<T>{
    fn item(&self) -> &T{
        &self.x
    }
}

struct B<V>{
    x : V,
    y : V,
}

struct C<U, V>{
    x : U,
    y : V
}

trait Shape<T>{
    fn area(&self) -> T;
}

struct Rectangle<T : Mul>{
    x : T,
    y : T,
}

// struct Circle<T :Mul>{
//     radius : T,
// }


impl <T: Copy> Shape<T> for Rectangle<T> where T : Mul<Output = T> {
    fn area(&self) -> T{
        self.x * self.y
    }
}

// impl <T : Mul<Output = T> + Copy> Shape<T> for Circle<T>{
//     fn area(&self) -> T{
//         3.14 * (self.radius * self.radius)
//     }
// }

fn main() {
    let s = Square{x : 10};
    println!("{:?}", s);
    let s = Square{x : 1.0};
    println!("{:?}", s);
    let s = Square{x : "10"};
    println!("{:?}", s);
    let s = Square{x : '0'};
    println!("{:?}", s);

    println!();

    p(10);
    p(String::from("String!"));
    let a = A { x : "Hello"};
    a.item();

    println!();

    let r = Rectangle{x : 10, y : 20};
    let r2 = Rectangle{x : 10.10, y : 20.31};

    println!("{}, {}", r.area(), r2.area());
}
