pub mod game {
    #[derive(PartialEq, Clone, Copy)]
    pub enum Player {
        X,
        Y,
        O
    }

    impl Default for Player {
        fn default() -> Self {
            Player::O
        }
    }

    pub struct Field {
        play_filed: [Player; 9],
    }

    impl Field {
        pub fn new() -> Field {
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
            self.play_filed[point] == Player::O
        }

        pub fn check_if_win(&self, player: &Player) -> bool {
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
}

#[cfg(test)]
mod field_test;