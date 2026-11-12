use crate::enums::shape::Shape;

pub fn calculate_area(shape: Shape) -> f64{
    if let Shape::Circle(radius) = shape{
        return radius * radius * 3.14;
    }

    if let Shape::Rectangle(width, height) = shape {
        return width * height;
    }

    if let Shape::Square(side) = shape {
        return side * side;
    }

    return 0.0;
}