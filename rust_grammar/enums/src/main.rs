#![allow(dead_code)] //사용하지 않는 코드에 대한 경고를 제거하는 annotation 

//trait : 공유 동작을 정의
//러스트 컴파일러에 특정한 타입이 갖고 다른 타입들과 함께 공유할 수도 있는 기능
//Interface와 유사한 기능.
//제너릭타입 Parameter를 사용하는 상황에서 Compile Time에 해당 제너릭 타입이 어떤 \
//트레잇을  구현한 타입이어야 함을 명시, 그러한 상황에서 우리가 사용하길 원하는 동작을 갖도록 \
//하기 위해 트레잇 바운드를 사용할 수 있음.
#[derive(Debug)]
enum Direction{
    Up(Point),
    Down(Point),
    Left(Point),
    Rigth(Point),
}

#[derive(Debug)]
enum Keys{
    UpKey(String),
    DownKey(String),
    LeftKey(String),
    RightKey(String),
}

//self : Method가 호출되고 있는 Struct의 Instance
impl Direction{
    //self -> Direction
    fn match_direction(&self) -> Keys {
        match *self{
            Direction::Up(_) => Keys::UpKey(String::from("Press w")),
            Direction::Down(_) => Keys::DownKey(String::from("Press s")),
            Direction::Left(_) => Keys::LeftKey(String::from("Press a")),
            Direction::Rigth(_) => Keys::RightKey(String::from("Press d")),
        }
    }
}

impl Keys{
    fn destruct(&self) -> &String{
        match *self {
            Keys::UpKey(ref s) => s,
            Keys::DownKey(ref s) => s,
            Keys::LeftKey(ref s) => s,
            Keys::RightKey(ref s) => s,
        }
    }
}

#[derive(Debug)]
struct Point{
    x : i32,
    y : i32,
}

enum Shape{
    Rectangle {width : u32, height : u32},
    Square(u32),
    Circle(f64),
}

impl Shape{
    fn area(&self) -> f64{
        match *self{
            Shape::Rectangle {width, height} => (width * height) as f64,
            Shape::Square(ref s) => (s * s) as f64,
            Shape::Circle(ref r) => 3.14 * (r * r),
        }
    }
}

fn division(x : f64, y : f64) -> Option<f64>{
    if y == 0.0{
        None
    }else{
        Some(x / y)
    }
}

fn main() {
    let r = Shape::Rectangle{width : 10, height : 70};
    let s = Shape::Square(10);
    let c = Shape::Circle(4.5);

    let u = Direction::Down(Point{x : 0, y : 1});
    let k = u.match_direction();
    let x = k.destruct();
    println!("march_direction : {:?}", k);
    println!("destruct : {}", x);

    let u = 10;
    let v = &u;
    let ref z = u;

    if z == v {
        println!("they are equal");
    }

    let ar = r.area();
    let aq = s.area();
    let ac = c.area();

    println!("{}\n{}\n{}", ar, aq, ac);

    let res = division(5.0, 7.0);
    match res{
        Some(x) => println!("{:.10}", x), //소수점 아래 10자리 까지
        None => println!("cannot divide by 0"),
    }
}
