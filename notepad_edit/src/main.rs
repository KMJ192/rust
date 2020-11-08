use std::fs::File;
use std::io::{self, *};

fn main() {

    let route = "D:\\develop\\toiproject";
    let name = "작성문서.txt";
    let file_route = route.to_owned() + "\\" + name;

    let mut f = File::open(file_route).expect("파일이 없음");
    let mut contents = String::new();

    f.read_to_string(&mut contents).expect("error");

    let mut replace_contents = contents.bytes().next();
    println!("{}", replace_contents.unwrap());

    let mut test = String::new();
    for c in contents.chars(){
        println!("{}", c);
    }
}
