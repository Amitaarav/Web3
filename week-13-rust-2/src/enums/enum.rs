use crate::enums::Direction;
use crate::enums::move_away::move_away;

pub fn direction_to_move(direction: &Direction) {
    let direction = match direction {
        Direction::Up => "UP",
        Direction::Down => "DOWN",
        Direction::Left => "LEFT",
        Direction::Right => "RIGHT",
    };
    move_away(direction.to_string());
}