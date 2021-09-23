#![allow(dead_code)]

pub mod bot;

pub mod play {
    use super::game::*;
    use crate::bot::play_bot;
    use std::io;
    use std::{thread, time};
    
    pub fn play_game() {
        let mut field = Field::new();
        let mut playing = Player::X;
        let mut computer = play_bot::Bot::new(Player::O);
        
        loop {
            println!("{}", field);
            match field.game_progress() {
                GameStatus::Win(player) => {
                    match player {
                        Player::None => println!("It is draw"),
                        _ => println!("Player {} won", player),
                    }
                    computer.end(player);
                    playing = Player::X;
                    field = Field::new();
                },
                GameStatus::Play => {
                    match playing {
                        Player::X => {
                            println!("Enter point:");
                            let mut point = String::new();
                            
                            io::stdin()
                            .read_line(&mut point)
                            .expect("Failed to read line");
                            
                            let point: usize = point.trim().parse().unwrap_or(1);
                            
                            if let Err(_) = field.one_play(&playing, point - 1) {
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


    pub fn play_bots() {
        let mut field = Field::new();
        let mut playing = Player::X;
        let mut bot_one = play_bot::Bot::new(Player::X);
        let mut bot_two = play_bot::Bot::new(Player::O);
        
        loop {
            match field.game_progress() {
                GameStatus::Win(player) => {
                    match player {
                        Player::None => println!("It is draw"),
                        _ => println!("Player {} won", player),
                    }
                    println!("{}", field);
                    thread::sleep(time::Duration::from_secs(1));

                    bot_one.end(player);
                    bot_two.end(player);
                    playing = Player::X;
                    field = Field::new();
                },
                GameStatus::Play => {
                    match playing {
                        Player::X => {
                            bot_one.play(&mut field);                            
                            playing = Player::O;
                        }
                        
                        _ => {
                            bot_two.play(&mut field);
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