fn main() {
//integer, unsigned integer
//i8, u8, i16, u16, i32, u32, i64, u64
//float
//f32, f64
//bool : true, false

    let x : u32= 5;
    let c : char = 'z';
    let t : (i32, f64, char) = (10, 10.1, 't');
    let (_, _, z) = t;
    println!("{}, {}, {:#?}, {}\n", x, c, t, z);
    
    let a = [1,2,3,4,5,6,7,8];
    let a1 = a[0];
    println!("a : {}\n", a1);

    let q = (1, 'a', false);
    let w = (2, (1, 'a', false));
    println!("q : {}\nw : {:?}\n", q.0, w.1);

    //i32타입으로 5개 배열 할당
    let _arr : [i32; 5] = [1,2,3,4,5];
    use std::mem; //memory관련 라이브러리
    println!("_arr : {}, _arr.length : {}, mem : {}\n", _arr[4], _arr.len(), mem::size_of_val(&_arr));
    let _arr2 = &_arr[2..4]; //_arr의 index 2부터 4까지의 data
    println!("{:?}\n", _arr2);

    let s = "String".to_string();
    let ss = String::from("String");
    println!("s : {}, ss : {}\n", s, ss);

    //slice string
    //String의 index부터 index 사이의 char slice
    let slice = &ss[0..4];
    println!("slice : {}\n", slice);

    let h = String::from("Hello, ");
    let w = String::from("World!");
    let s = h + &w; //(self, &String)형태
    println!("{}", s);

    //tuple 생성
    let tu = ();
    println!("tuple : {:?}", tu);

}
