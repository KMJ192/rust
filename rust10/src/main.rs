//메소드 문법

#[derive(Debug)]
struct Rectangle{
    length : u32,
    width : u32,
}

impl Rectangle { 
    //구현 : implementation 블록
    //Rectangle 인스턴스를 파라미터로 가지고 있는 area함수
    //self -> impl Rectangle 내용물 안에 위치하고 있어서 self의 타입은 Ractangle
    //소유권을 가져오지 않기 위함
    fn area(&self) -> u32{ 
        self.length * self.width
    }

    //self, &Rectangle(불변 참조자)
    //self -> rect1, other -> rect2 or rect3
    fn can_hold(&self, other: &Rectangle) -> bool{
        self.length > other.length && self.width > other.width
    }
    //연관함수
    //self파라미터를 가지고 있지 않는 함수도 impl 내에 정의 하는 것이 허용됨
    fn square(size : u32) -> Rectangle{
        Rectangle{length : size, width : size}
    }
}

fn main() {
    let rect1 = Rectangle{ length : 50, width : 30};
    let rect2 = Rectangle{ length : 10, width : 10};
    let rect3 = Rectangle{ length : 49, width : 29};
    let sq = Rectangle::square(3);
    
    println!("rect1 : {}", rect1.area());
    println!("rect2 : {}", rect1.can_hold(&rect2));
    println!("rect3 : {}", rect1.can_hold(&rect3));    
}
