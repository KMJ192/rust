
enum List{
    Cons(i32, Box<List>),
    End,
}

use List::Cons;
use List::End;

fn run<F>(f : F) where F : Fn(){
    f();
}

fn add<F>(f : F) -> i32 where F : Fn(i32) -> i32{
    f(3)
}

fn pr(){
    println!("normal function");
}

fn main() {
    let l = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(End))))));

    let y = 4;
    let x = &y;
    let z = Box::new(y);

    if *x == *z {
        println!("True");
    }

    let f = |i| i + 1;
    let g = 10;
    f(g);
    println!("{}", f(g));

    let p = || println!("this is a closure");
    p();

    let mut c = 0;
    let mut inc = || {
        c += 1;
        println!("incremented by 1: {}", c);
    };

    inc();
    inc();
    inc();

    let u = || println!("run function");
    run(u);

    let o = |i| i * 10;
    println!("{}", add(o));

    run(p);
    run(pr);

}
