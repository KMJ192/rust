fn main() {
    let mut s1 = String::from("te st");
    let word = first_word(&s1);
    println!("{}", word);
    s1.clear(); //String을 비워서 ""로 만듬
    //변수 word는 여기서부터 유효하지 않음
    //변수 s1과 word의 Sync가 맞지 않음

    //인자로 &str을 받을 경우 String, &str에 대한 같은 함수를 사용할 수 있음
    let _string1 = String::from("test test");
    let word1 = first_word(&_string1);
    let word2 = first_word(&_string1[..]);
    let _string2 = "test test";
    let word3 = first_word(_string2);
    let word4 = first_word(&_string2[..]);

    println!("{0}, {1}, {2}, {3}", word1, word2, word3, word4);


    let s2 = String::from("te st");
    let test1 = &s2[..2]; //start..end, start부터 시작하여 end를 포함하지 않는 연속 범위
    let test2 = &s2[3..];
    let test3 = &s2[..];
    println!("{0}, {1}, {2}", test1, test2, test3);

    let a = [1, 2, 3, 4, 5];
    let slice_arr = &a[1..3];
    println!("{}", slice_arr[0]);
}

// fn first_word(s : &String) -> usize{
//     let bytes = s.as_bytes();

//     //iter : collection각 요소를 반환하는 function
//     //enumrate : Tuple의 일부로 만들어 반환
//     //반환된 튜플의 첫번째 요소는 Index, 두번째 요소는 참조값
//     for(i, &item) in bytes.iter().enumerate(){
//         if item == b' '{ //byte리터러문법을 이용하여 공백값이 있을 경우 Index값 반환
//             return i;
//         }
//     }
//     s.len()
// }

//&str -> String, &str에 대한 같은 함수를 사용할 수 있음
fn first_word(s : &str) -> &str{
    let bytes = s.as_bytes();
    for(i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}
