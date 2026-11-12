use crate::structs::models::user::User;


pub fn is_allowed_to_vote(user: &User) -> bool {
    user.age >= 18
}
