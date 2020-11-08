#[derive(Debug)] //enum Example을 Debug로 사용할 수 있도록 하는 Annotation
enum Example{
    Float(f64),
    Int(i32),
    Text(String),
}

fn main() {
    let mut v : Vec<i32> = Vec::new();

    for i in 0 .. 5{
        v.push(i);
    }

    for i in &v{
        println!("{}", i);
    }

    println!("{:?}, {}, {}", &v, v.len(), v.capacity());
    println!("{:?}", v.pop());

    let r = vec![
        Example::Int(142),
        Example::Float(12.32),
        Example::Text(String::from("string")),
    ];

    println!("{:?}", &r);

    use std::collections::HashMap;

    let mut hm = HashMap::new();

    //Key, Value형태
    hm.insert(String::from("random"), 12);
    hm.insert(String::from("strings"), 49);

    for (k, v) in &hm{
        println!("{} : {}", k, v);
    }

    //hashmap에서 key에 맞는 Value 출력
    match hm.get(&String::from("random")){
        Some(&n) => println!("{}", n),
        _ => println!("no match"),
    }
    //hashmap에서 key에 대한 data제거
    hm.remove(&String::from("strings"));

    let s = Some('c');
    match s {
        Some(i) => println!("{}", i),
        _ => {}
    }
    if let Some(i) = s{
        println!("{}", i);
    }else{
        {}
    }

    let mut _s = Some(0);

    loop{
        match _s{
            Some(i) => if i > 19 {
                println!("Quit");
                _s = None;
            }else{
                println!("{}", i);
                _s = Some(i + 2);
            }, 
            _ => {
                break;
            }
        }
    }

    let f = 24.3232_f32;
    let i = f as u8;
    let c = i as char;
    println!("{}, {}, {}", f, i, c);

    println!("{}", 255 as char);

    use std::fs::File;

    let fi = File::open("test.txt");

    let fi = match fi{
        Ok(file) => file,
        Err(error) =>{
            panic!("No File {:?}", error);
        },
    };
}
