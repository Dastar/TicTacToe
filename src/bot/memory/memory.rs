use crate::game::Player;
//use crate::game::Field;
use crate::bot::memory::list::List;
use crate::bot::memory::Status;
//use crate::bot::memory::node::Node;

pub struct Memory {
    pub player: Player,
    pub lose: usize,
    pub win: usize,
    pub draw: usize,
    long_memory: List,
}

impl Memory {
    pub fn new(player: Player) -> Self {
        let long_memory = List::new();
        let lose = 0;
        let win = 0;
        let draw = 0;
        Memory { player, lose, win, draw, long_memory }
    }

    pub fn get_move(&mut self) -> usize {
        self.long_memory.get_move()
    }

    pub fn set_move(&mut self, movement: usize) {
        self.long_memory.set_move(movement);
    }

    pub fn end_game(&mut self, winner: Player) {
        if winner == self.player {
            self.long_memory.end_game(&Status::Win);
            self.win += 1;
        }
        else if winner == Player::None {
            self.long_memory.end_game(&Status::Draw);
            self.draw += 1;
        }
        else {
            self.long_memory.end_game(&Status::Lose);
            self.lose += 1;
        }
    }
}