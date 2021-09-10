use core::fmt;
use std::fmt::Formatter;
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Player {
    X,
    O,
    None
}

#[derive(PartialEq, Debug)]
pub enum GameStatus {
    Win(Player),
    Draw,
    Play
}

impl Default for Player {
    fn default() -> Self {
        Player::None
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Player::X => write!(f, "X"), 
            Player::O => write!(f, "O"), 
            Player::None => write!(f, " "), 

        }
    }
}

pub struct Field {
    play_filed: [Player; 9],
}

impl Field {
    pub fn new() -> Self {
        Field {
            play_filed: Default::default()
        }
    }

    pub fn one_play(&mut self, player: &Player, point: usize) -> Result<(), String> {
        if self.can_play(point) {
            self.play_filed[point] = player.clone();
        }
        else {
            return Err(String::from("One cannot play here"));
        }

        Ok(())
    }

    pub fn can_play(&self, point: usize) -> bool {
        self.play_filed[point] == Player::None
    }

    fn check_if_win(&self, player: &Player) -> bool {
        // Check every line and column if player won
        for i in 0..3 {
            let line_indexer = |index| i * 3 + index;
            let column_indexer = |index| i + index * 3;

            if self.check_index(player, line_indexer) || 
               self.check_index(player, column_indexer) {
                return true;
            }
        }

        // Check two other options to win
        let one_diag_indexer = |index: usize| index * 4;
        let two_diag_indexer = |index: usize| index * 2 + 2;
        if self.check_index(player, one_diag_indexer) ||
           self.check_index(player, two_diag_indexer) {
               return true;
           }

        false
    }

    pub fn game_progress(&self) -> GameStatus {
        if self.check_if_win(&Player::X) {
            return GameStatus::Win(Player::X);
        }

        if self.check_if_win(&Player::O) {
            return GameStatus::Win(Player::O);
        }

        if self.is_draw() {
            return GameStatus::Draw;
        }

        GameStatus::Play
    }

    fn is_draw(&self) -> bool {
        for i in self.play_filed {
            if i == Player::None {
                return false;
            }
        }

        true
    }

    fn check_index<T>(&self, player: &Player, indexer: T) -> bool
        where T: Fn(usize) -> usize {
        for i in 0..3 {                
            if self.play_filed[indexer(i)] != *player { 
                return false
            }
        }

        true
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut output = String::new();

        for i in 0..3 {
            for q in 0..3 {
                let point = format!("{}", self.play_filed[3 * i + q].to_string());
                output.push_str(&point);
                if q != 2 {
                    output.push_str(" | ");
                }
            }
            output.push('\n');
        }

        write!(f, "{}", output)
    }
}