//참조자 / 빌림
fn main() {
    let s1 = String::from("References");    
    let lent = string_length(&s1); //& -> 참조(소유x)
    println!("{}", lent);

    //mut Keyword를 붙일 경우 참조자의 값을 변경할 수 있음
    let mut s2 = String::from("References");
    refer_change(&mut s2); //가변 참조자

    let mut s3 = String::from("test");
    let _r1 = &s3;
    let _r2 = &s3;
    let _r3 = &mut s3;

    //Dangling Refereneces(댕글림 참조자)
    //메모리를 가리키는 포인터가 있는데 포인터 해제
    let dangling_test = dangling();
    println!("{}", dangling_test);
}

fn dangling() -> String{
    let s = String::from("test"); //String의 참조자 반환
    //&s //Scope를 벗어나고 메모리가 헤제됨, Compile Error
    s //String을 직접 반환
}

fn string_length(s : &String) -> usize{
    s.len()
}

fn refer_change(s : &mut String){
    s.push_str(", test");
    println!("{}", s);
}