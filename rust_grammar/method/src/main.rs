use std::fmt;

#[derive(Debug)] //Object Struct를 Debug -> {:?}로 println!("{:?}") 출력 가능
struct Object{
    width : u32,
    height : u32,
}

impl fmt::Display for Object{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "({} {})", self.width, self.height)
    }
}

//Methods
impl Object{
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn show(&self){
        println!("{}*{} width area : {}", self.width, self.height, self.area());
    }
}

//Related Function
impl Object{
    fn new(width: u32, height : u32) -> Object {
        Object{
            width,
            height,
        }
    }
}

// fn area(obj : &Object) -> u32 {
//     obj.width * obj.height
// }

fn main() {
    let o = Object{
        width: 35,
        height : 55,
    };
    
    //println!("{}*{} width area : {}", o.width, o.height, area(&o));
    println!("{}*{} width area : {}", o.width, o.height, o.area());
    let obj = Object::new(57, 83);
    println!("{}*{} width area : {}", obj.width, obj.height, obj.area());

    o.show();
    obj.show();

    println!("{:?}", o);
    println!("{:?}", obj);

}