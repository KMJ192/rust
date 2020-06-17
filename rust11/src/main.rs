//열거형 정의
enum IpAddrKind{
    V4,
    V6
}
enum IpAddr{
    V4(String),
    V6(String)
}
enum IpAddress{
    V4(u8, u8, u8, u8),
    V6(String)
}
enum Message{
    Quit,
    Move{x : i32, y : i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}
//Option 열거형
//Null참조에 대한 방지
enum Option<T>{
    Some(T),
    None,
}
fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    let home2 = IpAddress::V4(127, 0, 0, 1);
    let loopback2 = IpAddress::V6(String::from("::1"));

    impl Message{
        fn call(&self){
            //Method 내용 정의 가능
        }
    }
    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_stirng = Some("String");
    
}
fn route(ip_type : IpAddrKind){}