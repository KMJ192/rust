//match 흐름 제어 연산자
//하나의 Patter만 matching 시키고 나머지 경우는 무시
#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn main() {
    let _quarter = Coin::Quarter(UsState::Alabama);
    value_in_cents(_quarter);
    
    let five = Some(5);
    let six = oneplus(five);
    let none = oneplus(None);

    //if let의 사용
    let _quarter2 = Coin::Quarter(UsState::Alabama);
    let mut count = 0;
    if let Coin::Quarter(state) = _quarter2 {
        println!("State quarter from {:?}!", state);
    }else{
        count += 1;
    }
    println!("{}", count);
}

fn oneplus( x : Option<i32>) -> Option<i32>{
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn value_in_cents(coin : Coin) ->u32{
    match coin{
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn _placeholder(_val : u8){
    match _val {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), //변경자(placeholder) 어떠한 값과도 matcing이 됨
    }
}
