use std::env;

fn main(){
    //커맨드Line 인자 허용
    let args : Vec<String> = env::args().collect(); 

    let query = &args[1];
    let filename = &args[2];

    println!("{:?},  Searching for {}, In file {}", args, query, filename);
}