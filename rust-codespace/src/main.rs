use crate::data::user::User;
use crate::data::todo::Todo;
pub mod data;

fn main() {
    let u1 = User{
        user_id: 1,
        username: String::from("Amit Kumar"),
        password: String::from("123123")
    };

    let todos = vec![Todo {
        user_id: 1,
        title: String::from("Go to Gym"),
        description: String::from("Go to Gym at 5")
    }];

    print!("{:?}", u1);
    print!("{:?}", todos);
}
