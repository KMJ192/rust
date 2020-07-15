enum Result<T, E>{
    Ok(T),
    Err(E),
}
// enum Option<T>{
//     Some(T),
//     None,
// }

fn exit(x : i32){
    if x == 0{
        panic!("error!");
    }
    println!("{}", x);
}

fn mat(x : Option<i32>){
    match x{
        Some(0) => panic!("panic 0"),
        Some(x) => println!("{}", x),
        None => println!("None"),
    }
}

use std::fs::File;
use std::io::ErrorKind;

fn main(){
    exit(1);
    //exit(0);

    mat(Some(1));
    mat(None);
    //mat(Some(0));

    let f = File::open("text.txt");
    let f = match f{
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("text.txt"){
                Ok(fc) => fc,
                Err(e) => {
                    panic!("could not create file : {:?}", e)
                },
            }
        },
        Err(error) => {
            panic!("could not open the file : {:?}", error)
        },
    };

    let z = File::open("test.txt").expect("Could not open file");
}

