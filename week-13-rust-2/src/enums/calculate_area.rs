use crate::enums::shape::Shape;

pub fn calculate_area(shape: Shape) -> f64{
    // if let Shape::Circle(radius) = shape{
    //     return radius * radius * 3.14;
    // }

    // if let Shape::Rectangle(width, height) = shape {
    //     return width * height;
    // }

    // if let Shape::Square(side) = shape {
    //     return side * side;
    // }

     return match shape {
        Shape::Circle(radius) => {
            3.14 * radius * radius
        },

        Shape::Rectangle(width, height ) => {
            width * height
        },

        Shape::Square(side) => {
            side * side
        }
    };

    // return area;
}