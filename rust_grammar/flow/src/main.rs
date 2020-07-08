
fn main() {
    let n = 2;

    if (n % 4) == 0{
        println!("0");
    }else if n % 3 == 0 {
        println!("1");
    }else if n % 2 == 0 {
        println!("2");
    }else{
        println!("3");
    }
    println!();
    let c = true;
    let n = if c /*== false*/ {
        70 //c가 true 일때
    }else{
        //60 //c가 false 일때
        "60".parse::<i32>().unwrap() // Type Casting가능
    };

    println!("{}", n);
    println!();
    let mut t = 0;
    loop{
        println!("{}", t);
        t += 1;

        if t >= 10 {
            break;
        }
    }
    let mut z = 10;

    while z != 0{
        println!("{}", z);
        z -= 1;
    }
    println!();
    'a : loop{
        println!("a");
        'b : loop{
            println!("b");
            'c : loop{
                println!("c");
                break 'a;
            }
        }
    }
    println!();
    let mut x = 10;
    let mut v : Vec<i32> = Vec::new(); 
    while v.len() < 5{
        v.push(x);
        x += 10;
    }
    for i in v {
        println!("i1 : {}", i);
    }
    println!();
    for i in 0..=10 /* ..= 마지막 포함*/ {
        println!("i2 : {}", i);
    }
    println!();
    match x {
        1 => println!("1"),
        2 => println!("2"),
        3 => println!("3"),
        4 => println!("4"),
        5 | 6 => println!("5 또는 6"),
        59 ..= 61 => println!("59 ~ 61 사이"),
        _ => println!("default"),
    }
    println!();
    let pair = (0, 2);
    match pair {
        (1, y) => println!("y : {}", y),
        (x, 2) => println!("x : {}", x),
        _ => println!("default"),
    }

    let pair2 = (5, -5);
    match pair2{
        (x, y) if x == y => println!("equal"),
        (x, y) if x + y == 0 => println!("equal Zero"),
        (x, _) if x % 2 == 0 => println!("equal"),
        _ => println!("default"),
    }

    let p = 16;
    match p {
        n @ 1 ..= 12 => println!("n : {}", n),
        n @ 13 ..= 19 => println!("n : {}", n),
        _ => println!("default"),
    }
    println!();

    let n = match p {
        n @ 5 => n,
        n @ 13 ..= 19 => n,
        _ => 0,
    };
    println!("n2 : {}", n);
}
