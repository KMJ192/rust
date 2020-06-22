use std::fmt::Display;

struct ImportantExcerpt<'a>{
    part : &'a str,
}

fn main(){
    let a;
    {
        let b = 4;
        a = &b; //빌림 검사기
        println!("a : {}", a);
        //a는 scope밖에서 사용 불가능
    }

    let x = 5;
    let r = &x;
    println!("r : {}", r);

    let string1 = String::from("abcd");
    {
        let string2 = "xyz";
        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a 'a'");
    let i = ImportantExcerpt{
        part : first_sentence
    };

    impl<'a> ImportantExcerpt<'a>{
        //파라미터는 오직 self에 대한 참조자, return값은 그저 i32
        fn level(&self) -> i32{
            3
        }
        
        //2개의 입력 라이프타임 -> &self와 announcement에 각각 라이프타임 부여
        //파라미터 중 하나가 &self이므로 반환타입은 &self의 라이프 타임을 얻음
        fn announce_and_return_part(&self, announcement : &str) -> &str{
            println!("Attention please : {}", announcement);
            self.part
        }
    }
}

//반환 타입에 대하여 제너릭라이프타임 파라미터가 필요함
//반환되는 참조자가 x를 참조하는지 y를 참조하는지 러스트가 알 수 없음
// fn longest(x: &str, y : &str) -> &str{
//     if x.len() > y.len() {
//         x
//     }else{
//         y
//     }
// }

//함수 시그니처 내의 Life Time 명시
//제너릭 Life Time 파라미터 -> parameter들과 반환 값에서의
//참조자들에 대하여 제약사항은 parameter모두 동일한 라이프타임을 갖고 있어야 함
//라이프타임 'a만큼 살아있는 String Slice
fn longest<'a>(x: &'a str, y : &'a str) -> &'a str{
    if x.len() > y.len() {
        x
    }else{
        y
    }
}

//라이프타임 생략
fn first_word(s: &str) -> &str{
    let bytes = s.as_bytes();
    for(i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}

fn longest_with_an_announcement<'a, T>(x : &'a str, y : &'a str, ann : T) -> &'a str
    where T : Display
{
    println!("Announcement {}", ann);
    if x.len() > y.len(){
        x
    }else{
        y
    }
}