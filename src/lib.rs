#![allow(dead_code)]

pub mod bot;

pub mod play {
    use super::game::*;
    use crate::bot::play_bot;
    use std::io;
    
    pub fn play_game() {
        let mut field = Field::new();
        let mut playing = Player::X;
        let computer = play_bot::Bot::new(Player::O);
        
        loop {
            println!("{}", field);
            match field.game_progress() {
                GameStatus::Draw => {
                    println!("It is draw");
                    break;
                },
                GameStatus::Win(player) => {
                    println!("Player {} won", player);
                    break;
                },
                GameStatus::Play => {
                    match playing {
                        Player::X => {
                            println!("Enter point:");
                            let mut point = String::new();
                            
                            io::stdin()
                            .read_line(&mut point)
                            .expect("Failed to read line");
                            
                            let point: usize = point.trim().parse().expect("Please type a number!");
                            
                            if let Err(_) = field.one_play(&playing, point) {
                                println!("Cant go here");
                                continue;
                            }
                            
                            playing = Player::O;
                        }
                        
                        _ => {
                            computer.play(&mut field);
                            playing = Player::X;
                        }
                    }
                    
                },
            }
        }
}

}
pub mod game;

#[cfg(test)]
mod field_test;