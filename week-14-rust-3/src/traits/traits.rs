enum Shape<T> {
    Circle(T),
    Rect(T, T),
    Square(T),
}

pub fn traits() {
    let v = vec![10,20,30];
    let x: Option<i32> = v.into_iter().nth(0);

    let r1 = Shape::Rect(1.0, 2.0);

}