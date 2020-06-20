enum SpreadsheetCell{
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut v: Vec<i32> = Vec::new();
    for i in 0..5{
        v.push(i);
    }
    //벡터 v의 요소 탐색
    for i in &v {
        println!("{}", i);
    }
    println!("{:?}", v);
    let v_val1 = v.get(0);
    let v_val2 = &v[1];
    
    let mut v1 = vec![100, 200, 300];
    //vector v1의 각 요소에 50 add
    for i in &mut v1{
        *i += 50;
    }
    println!("v1 : {:?}", v1);

    //enum을 이용하여 vector에 Data 입력
    let row = vec![SpreadsheetCell::Int(3), 
                        SpreadsheetCell::Text(String::from("Blue")),
                        SpreadsheetCell::Float(10.10)
                        ];
}