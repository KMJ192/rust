use std::ops; //연산자 오버로딩

struct A;
struct B;
#[derive(Debug)]
struct AB;
#[derive(Debug)]
struct BA;

impl ops::Add<B> for A{
    type Output = AB;
    fn add(self, _rhs:B) -> AB{
        AB
    }
}

impl ops::Add<A> for B{
    type Output = BA;

    fn add(self, _rhs : A) -> BA{
        BA
    }
}

struct A1 {
    a : String,
}

impl Drop for A1{
    fn drop(&mut self){
        println!("dropped {}", self.a)
    }
}

struct Fib{
    c : u32,
    n : u32,
}

impl Iterator for Fib{
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let n = self.c + self.n;
        self.c = self.n;
        self.n = n;

        Some(self.c)
    }
}

fn fib() -> Fib {
    Fib{c : 1, n : 1}
}

fn main() {
    println!("{:?}", A + B);
    println!("{:?}", B + A);
    println!();
    let a = A1{a : String::from("A")};
    {
        let b = A1{a : String::from("B")};
        {
            let c = A1{a : String::from("C")};

            println!("scope 2");
        }
        println!("scope 1");
    }
    drop(a);
    println!("ending");

    println!();
    for j in fib().take(10){
        println!("{}", j);
    }

    println!();

    for j in fib().skip(14).take(10){
        println!("{}", j);
    }

    let mut f = fib();

    println!("{:?}", f.next());
    println!("{:?}", f.next());
    println!("{:?}", f.next());
    println!("{:?}", f.next());
    println!("{:?}", f.next());
}
