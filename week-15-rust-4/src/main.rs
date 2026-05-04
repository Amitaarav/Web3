use std::f32::consts::PI;
trait Shape {
    // function declaration
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32;
}

struct Rect{
    width: f32,
    height: f32
}

struct Circle{
    radius: f32
}

impl Shape for Rect{
    // function Definition
    fn area(&self) -> f32{
        self.width * self.height
    }
    fn perimeter(&self) -> f32{
        (self.width + self.height) * 2.0
    }
}

impl Shape for Circle{
    fn area(&self) -> f32{
        PI * self.radius * self.radius
    }

    fn perimeter(&self) -> f32{
        2.0 * PI * self.radius
    }
}

macro_rules! say_hello{
        () => {
            println!("Hello World!");
        }
    }

macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name(){
            println!("Hello from {} ", stringify!($func_name));
        }
    }
}
create_function!(hello);

macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name(){
            println!("Hello from {} ", stringify!($func_name));
        }
    }
}

//

fn main() {
    let r = Rect {
        width: 10.0,
        height: 20.0
    };

    let c = Circle{
        radius: 10.0
    };

    println!("{}", get_area_and_perimeter(r).0);
    println!("{}", get_area_and_perimeter(c).0);

    //
    say_hello!();
    hello();
}

// fn get_area_and_perimeter(s: impl Shape) -> (f32, f32){
//     (s.area(), s.perimeter())
// }

fn get_area_and_perimeter<T: Shape>(s: T) -> (f32, f32){
    (s.area(), s.perimeter())
}