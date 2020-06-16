//구조체를 이용하는 예제 프로그램
//사각형의 넓이를 계산하는 프로그램
fn main() {
    //일반적인 변수 표현
    let length1 = 50;
    let width1 = 30;
    let area1 = area(length1, width1);
    println!("length : {}, width : {}, area : {}", length1, width1, area1);

    //튜플을 이용한 리팩터링
    let rect1 = (50, 30);
    println!("length : {}, width : {}, area : {}", rect1.0, rect1.1, fn_rect(rect1));
    
    //구조체를 이용한 리팩터링
    let rect2 = Rectangle{
        length : 50,
        width : 30
    };
    println!("length : {}, width : {}, area : {}", rect2.length, rect2.width, rect2.length*rect2.width);
    println!("rect1 is {:?}", rect2); //Debug formatting
}

#[derive(Debug)] //Debugging 정보 포함시키는 annotation
struct Rectangle{
    length : u32,
    width : u32,
}
fn area(length:u32, width : u32) -> u32{
    length * width
}
fn fn_rect(dimension : (u32, u32)) -> u32{
    dimension.0 * dimension.1
}