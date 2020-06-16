fn main(){
    let mut __string = String::from("Test String"); //String변수 선언 및 Input Data
    __string.push_str(", Test String2"); //String의 뒤에 String붙이기
    println!("{}", __string);

    //Data 이동
    //x의 내용을 y로 이동, x의 내용은 지워짐
    let x = String::from("StringMove");
    let y = x;

    //println!("{}", x); //x에 내용이 없어서 Coplie Error
    println!("{}", y);

    //s1의 내용을 s2로 복사
    //s1의 내용을 보존하면서 s2에 내용을 복사하는 함수 clone()
    let s1 = String::from("StringCopy");
    let s2 = s1.clone();

    println!("{0}, {1}", s1, s2);

    let _s1 = String::from("StringOwnerShip");
    takes_ownsership(_s1);

    let _s2 = 5;
    make_copy(_s2);

    let str1 = _str_return();
    let _to_str2 = String::from("입력");
    let str2 = _param_return(_to_str2);
    
    println!("{0}, {1}", str1, str2);

    //다중 return값
    let in_str = String::from("String 및 길이 출력");
    let (re_str, in_len) = _str_len(in_str);

    println!("Get Length{}, {}", re_str, in_len);

}

fn takes_ownsership(_gs : String){
    println!("{}", _gs);
}

fn make_copy(_in : i32){
    println!("{}", _in);
}

//리턴
fn _str_return() -> String{
    let rs = String::from("String을 return");
    rs
}

fn _param_return(_ps : String) -> String{
    _ps
}

fn _str_len(s : String) -> (String, usize){
    let s_len = s.len();
    (s, s_len)
}