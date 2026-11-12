mod enums;
mod structs;

use structs::models::address::Address;
use structs::models::user::User;
use structs::services::voting::is_allowed_to_vote;
use structs::services::know::who_am_i;
use enums::direction::Direction;
use enums::direction_to_move::direction_to_move;
use enums::calculate_area::calculate_area;
use enums::shape::Shape;

fn main() {
    let user = User {
        name: String::from("Amit"),
        age: 24,
        addresses: vec![
            Address {
                city: String::from("Allahabad"),
                pincode: String::from("212109"),
                country: String::from("India"),
            },
        ],
        birthdays: vec![12, 5, 2000],
    };

    println!("Name: {}", user.name);
    println!("Age: {}", user.age);
    println!("City: {}", user.addresses[0].city);
    println!("Pincode: {}", user.addresses[0].pincode);
    println!("Country: {}", user.addresses[0].country);
    println!("birthdays: {:?}", user.birthdays);
    println!("Can vote: {}", is_allowed_to_vote(&user));
    println!("Know me: {}", who_am_i());

    // enums

    direction_to_move(&Direction::Down);

    // calculate area

    let rectangle = Shape::Rectangle(4.0, 5.0);

    println!("Area of rectangle: {:?}", calculate_area(rectangle));
}