#![allow(dead_code)] //사용하지 않는 코드에 대한 경고 제거

//trait : 특정한 타입이 갖고 다른 타입들과 함께 공유할 수도 있는 기능

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

fn main() {
    let u = Direction::Up(Point{x : 0, y : 1});
    let k = u.match_direction();
    let x = k.destruct();
    println!("{:?}", k);
    println!("{}", x);

    let u = 10;
    let v = &u;
    let ref z = u;

    if z == v {
        println!("thea are equal");
    }
    
}
