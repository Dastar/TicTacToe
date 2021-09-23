use crate::game;
use crate::bot::memory::memory::Memory;
pub struct Bot {
    memory: Memory
}

impl Bot {
    pub fn new(player: game::Player) -> Self {
        Bot { memory: Memory::new(player) }
    }
    
    pub fn play(&mut self, field: &mut game::Field) {
        self.save_move(field);
        self.play_move(field);
    }

    pub fn end(&mut self, winner: game::Player) {
        self.memory.end_game(winner);
    }

    fn save_move(&mut self, field: &mut game::Field) {
        let last_turn = field.last_turn;
        if let Some(movement) = last_turn {
            self.memory.set_move(movement);
        }
    }

    fn play_move(&mut self, field: &mut game::Field) {
        let point = self.memory.get_move();
        field.one_play(&self.memory.player, point).unwrap_or_default();
    }
}