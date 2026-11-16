struct Rect<T> {
    width: T,
    height: T,
}

// get_area
// fn get_area(v: Rect<u32>) -> u32{

// }

// fn get_area(v: Rect<f32>) -> f32{
    
// }

fn get_area<T: std::ops::Mul<Output = T>>(v: Rect<T>) -> T{
    return v.width * v.height;
}

pub fn generics_structs(){
    let r1 = Rect {
        width: 20,
        height: 40,
    };

    let r2 = Rect {
        width: 30.0,
        height: 40.0,
    };

    println!("{}", get_area(r1));
    println!("{}", get_area(r2));
}