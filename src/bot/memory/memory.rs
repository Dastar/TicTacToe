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
    memory: List,
    turns: i32
}

impl Memory {
    pub fn new(player: Player) -> Self {
        let memory = List::new();
        let lose = 0;
        let win = 0;
        let draw = 0;
        let turns = 0;
        Memory { player, lose, win, draw, memory, turns }
    }

    pub fn get_move(&mut self) -> usize {
        self.turns += 1;
        self.memory.get_move()
    }

    pub fn set_move(&mut self, movement: usize) {
        self.memory.set_move(movement);
    }

    pub fn end_game(&mut self, winner: Player) {
        let status: Status;
        if winner == self.player {
            status = Status::Win;
            self.win += 1;
        }
        else if winner == Player::None {
            status = Status::Draw;
            self.draw += 1;
        }
        else {
            status = Status::Lose;
            self.lose += 1;
        }

        self.memory.end_game(&status, self.turns);
        self.turns = 0;

    }
}