use crate::game::Player;

enum Status {
    Move(i8),
    StartGame,
    Win,
    Lose,
    Draw
}

struct Memory {
    status: Status,
    moves: Box<[Memory; 9]>,
    active: bool,
    weight: i32,
    my_move: bool,
    play: Player
}

impl Memory {
    fn new(my_move: bool) { 
        for i in 0..9 {

        }
    }
}