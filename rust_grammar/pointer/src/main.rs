
fn create() -> Box<Fn()> {
    //Box : heap 할당을 위한 포인터의 유형
    Box::new(move || println!("this is a closure in a box"))
}

trait Iterator{
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

fn is_even(n : u32) -> bool{
    n % 2 == 0
}

fn main(){
    let x = create();
    x();

    let v = vec![1, 2, 3];
    println!("v {}", v.iter().any(|&x| x != 2));
    for i in v.iter(){
        println!("{}", i);
    }

    v.iter().next();

    println!();

    let top = 1000;
    let mut c = 0;

    for n in 0.. {
        let x = n * n;
        if x >= top{
            break;
        }else if is_even(x){
            c += x;
        }
    }
    println!("{}", c);

    let s: u32 = 
    (0..).map(|n| n*n)
    .take_while(|&n| n < 10000)
    .filter(|&n| is_even(n))
    .fold(0, |s, i| s + i);

    println!("{}", s);
}