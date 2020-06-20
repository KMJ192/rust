//이름 가져오기Import Names

pub mod a {
    pub mod series{
        pub mod of{
            pub fn nested_modules(){println!("Rust");}
        }
    }
}

enum TrafficLight {
    Red,
    Green,
    Blue
}

//use를 사용 및 아래의 코드에서 function 직접 참조
use a::series::of::nested_modules;
use TrafficLight::*;

fn main() {
    //code 가독성 하락
    //a::series::of::nested_modules();

    nested_modules();
    let red = Red;
    let yellow = Yellow;
    let green = Green;
}
