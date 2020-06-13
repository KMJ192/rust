fn main() {
    let _num: u32 = "42".parse().expect("Not a number");
    let _num1 = 2.0; //f64
    let _num2 : f32 = 3.0;
    println!("{}", _num);

    //tuplization
    //복합 Type
    let tup : (i32, f64, u8) = (100, 10.1, 1);
    let (_x, _y, _z) = tup;
    println!("tup first : {}", _x);
    let _first = tup.0;
    let _second = tup.1;
    let _third = tup.2;

    println!("tuple 출력 : {0}, {1}, {2}", _first, _second, _third);

    //배열 선언 및 요소 접근
    let arr = [1, 2, 3, 4, 5];
    let first_arr = arr[0];
    let second_arr = arr[1];
    let third_arr = arr[2];

    println!("arr[0] : {0}, arr[1] : {1}, arr[2] : {2}", first_arr, second_arr, third_arr);

    another_function(arr[0]);

    //구문과 표현식
    let _construction = 10; //구문
    let _expression = { //표현식
        let _construction = 5;
        _construction + 1 //세미콜론 입력하면 안됨
    };

   println!("구문 : {1}, 표현식 : {0}", _expression, _construction);

   //반환 값을 가지는 function
   let refn = five(5);
   println!("반환값을 가지고 있는 함수 five() : {}", refn);

   //제어문
   // if문
   let connum = 5;
   if connum > 5{
       println!("5보다 크다");
   }else if connum == 5{
        println!("5와 같다");
   }
   else{
       println!("5보다 작다");
   }

   let cwb = true;
   let cwbnum = if cwb {
       5
   }else{
       6
   };
   println!("{}", cwbnum);

   let mut loopnum = 3;
   // while 문
   while loopnum != 0{
       println!("{}", loopnum);
       loopnum = loopnum - 1;
   }
   // for 문
   for i in (1..4){
        println!("i : {}", i);
   }
   //역순 rev()
   for i in (1..4).rev(){
    println!("역순 i : {}", i);
}
   //배열 iterator
   for i in arr.iter(){
       println!("배열 순회 : {}", i);
   }
}

fn another_function(x: i32){
    println!("Parameter Value : {}", x);
}

fn five(param : i32) -> i32{
    param //세미콜론 금지, 함수의 구문을 산출하지 않게 되는 현상
}

