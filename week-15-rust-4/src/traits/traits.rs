trait Shape {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32;
}

struct Rect {
    width: f32,
    height: f32,
}

struct Circle {
    radius: f32,
}

impl Shape for Rect {
    fn area(&self) -> f32{
        return self.width * self.height;
    }

    fn perimeter(&self) -> f32{
        return 2.0 * (self.width + self.height)
    }
}

impl Shape for Circle {
    fn area(&self) -> f32{
        return 3.14 * self.radius * self.radius;
    }

    fn perimeter(&self) -> f32{
        return 2.0 * 3.14 * self.radius;
    }
}


fn get_area_and_perimeter<T: Shape>(shape: T) -> (f32, f32) {
    return (shape.area(), shape.perimeter());
}

pub fn trait_funtion(){
    let r1 = Rect {
        width: 20.0,
        height: 40.0,
    };

    println!("area and perimeter of Rectangle {:?}", get_area_and_perimeter(r1));
}