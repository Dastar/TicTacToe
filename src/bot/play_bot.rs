use crate::game;
pub struct Bot {
    player: game::Player
}

impl Bot {
    pub fn new(player: game::Player) -> Self {
        Bot { player }
    }
    
    pub fn play(&self, field: &mut game::Field) {
        for index in 0..9 {
            if field.can_play(index) {
                if let Ok(_) = field.one_play(&self.player, index) {
                    break;
                }
            }
        }
    }
}