//구조체 정의 및 초기화
struct User{
    username : String,
    email : String,
    sign_in_count : u64,
    active : bool,
}

//필드의 이름을 제외하고 타입만 정의 하는 경우
struct Color(i32, i32, i32);

fn main() {
    //정의한 구조체에 각 필드의 값을 명세한 instance 생성
    let user1 = User{
        username : String::from("kmj"),
        email : String::from("kmj@google.com"),
        sign_in_count : 1,
        active : true, 
    };

    println!("username1 : {}", user1.username);
    println!("email1 : {}", user1.email);
    println!("sign_in_count1 : {}", user1.sign_in_count);
    println!("active1 : {}\n", user1.active);

    //구조체 갱신법으로 기존 구조체 인스턴스로 새 구조체 인스턴스 생성
    let user2 = User{
        username : String::from("kmj2"),
        email : user1.email.clone(),
        active : user1.active,
        sign_in_count : user1.sign_in_count
    };
    
    println!("username2 : {}", user2.username);
    println!("email2 : {}", user2.email);
    println!("sign_in_count2 : {}", user2.sign_in_count);
    println!("active2 : {}", user2.active);

    let black = Color(0,0,0);
}

fn build_user(email: String, username : String) -> User{
    //변수명이 필드명과 같을 경우, 필드 초기화 축약법(field init shorthand)
    User{
        username,
        email,
        active : true,
        sign_in_count : 1,
    }
    
}
