pub fn generics(){
    println!("{}", generics_mul(10, 20));
    println!("{}", generics_mul(10, 20));
    println!("{}", generics_mul(10.0, 20.0));
    println!("{}", generics_mul(10.0, 20.0));
}

fn mul_u32(a: u32, b: u32) -> u32{
    return a * b;
}

fn mul_i32(a: i32, b: i32) -> i32{
    return a * b;
}

fn mul_f32(a: f32, b: f32) -> f32{
    return a * b;
}

fn mul_f64(a: f64, b: f64) -> f64{
    return a * b;
}

fn generics_mul<T: std::ops::Mul<Output = T>>(a: T, b: T) -> T {
    return a * b;
}