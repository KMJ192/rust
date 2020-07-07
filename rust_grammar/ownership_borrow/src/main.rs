fn take(v : Vec<i32>){
    println!("v : {}", v[10] + v[100]);
}

fn cop(a : i32, b : i32){
    println!("a + b : {}", a + b);
}

fn re (v : Vec<i32>) -> Vec<i32> {
    println!("v : {}", v[10] + v[12]);
    v
}

fn borrow1(v : &Vec<i32>){
    println!("v : {}", (*v)[10] + (*v)[12]);
}

fn borrow2(v : &Vec<i32>){
    println!("v : {}", v[10] + v[12]);
}
fn main() {
    let x = 1;
    let y = x;
    {
        let a = 10;
        println!("{}", a);
    }
    println!("{}, {}\n", x, y);

    let mut v = Vec::new();
    for i in 1..1000{
        v.push(i);
    }

    //take(v);
    v = re(v);
    borrow1(&v); //주소값만 참조
    borrow2(&v);

    let a = 11;
    let b = 22;
    cop(a, b);
    println!("a : {}, b: {}", a, b);

    let _vec = vec![3,3,5,4,5,9,1,2,0,5,7,4,4,3,3,8];
    for &i in &_vec{
        let r = _count(&_vec, i);
        println!("{} is repeadted {} times", i, r);
    }
}

fn _count(v : &Vec<i32>, val : i32) -> usize{
    //vector를 순회하며 x의 값과 value값이 동일할 경우 해당 Count를 반환
    v.into_iter().filter(|&&x| x == val).count()
}
//오너쉽, 대여