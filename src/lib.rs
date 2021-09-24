#![allow(dead_code)]

pub mod bot;

pub mod play {
    use super::game::*;
    use crate::bot::play_bot;
    use std::io;
    use std::{thread, time};

    const TOTAL_GAMES: usize = 400000;
    const STATISTIC_STEP: usize = 20000;
    struct  Bots {
        bot_x: play_bot::Bot,
        bot_o: play_bot::Bot,
    }

    impl Bots {
        fn new() -> Self {
            let bot_x = play_bot::Bot::new(Player::X);
            let bot_o = play_bot::Bot::new(Player::O);
            Bots { bot_x, bot_o }
        }
    }
    
    pub fn play() {
        let mut bots = Bots::new();
        play_bots(&mut bots);
        play_game(&mut bots.bot_x);
    }

    fn parse(c: char) -> usize {
        match c {
            'q' => 1,
            'w' => 2,
            'e' => 3,
            'a' => 4,
            's' => 5,
            'd' => 6,
            'z' => 7,
            'x' => 8,
            'c' => 9,
            _ => 1,
        }
    }

    pub fn play_game(computer: &mut play_bot::Bot) {
        let mut field = Field::new();
        let mut playing = Player::X;
        //let mut computer = play_bot::Bot::new(Player::O);
        
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
                        Player::O => {
                            if !human_play(&mut field, &playing) {
                                continue;
                            }        
                            computer.save_move(&mut field);                    
                        }
                        
                        _ => {
                            computer.play(&mut field);  
                        }
                    }

                    playing = swap_players(playing);
                    
                },
            }
        }
    }

    fn swap_players(p: Player) -> Player {
        match p {
            Player::X => Player::O,
            _ => Player::X
        }
    } 

    fn human_play(field: &mut Field, playing: &Player) -> bool {
        println!("Enter point:");
        let mut point = String::new();
       
        io::stdin()
        .read_line(&mut point)
        .expect("Failed to read line");
       
        let point: char = point.trim().parse().unwrap_or('q');
       
        if let Err(_) = field.one_play(&playing, parse(point) - 1) {
            println!("Cant go here");
            return false;
        }
        true
    }

    fn print(output: &bool, field: &Field) {
        if *output {
            println!("{}", field);
        }
    }

    fn play_bots(bots: &mut Bots) {
        let mut field = Field::new();
        let mut playing = Player::X;
        let mut total_games: usize = 0;
        let output = false;
        loop {
            match field.game_progress() {
                GameStatus::Win(player) => {
                    total_games += 1;
                    if total_games % STATISTIC_STEP == 0 {
                        bots.bot_x.statistics(total_games);
                    }

                    // match player {
                    //     Player::None => println!("It is draw"),
                    //     _ => println!("Player {} won", player),
                    // }
                    // println!("{}", field);

                    //thread::sleep(time::Duration::from_millis(100));
                    bots.bot_x.end(player);
                    bots.bot_o.end(player);
                    playing = Player::X;
                    field = Field::new();

                    if total_games == TOTAL_GAMES {
                     //   output = true;
                        break;
                    }
                },
                GameStatus::Play => {
                    match playing {
                        Player::X => {
                            bots.bot_x.play(&mut field);   
                            bots.bot_o.save_move(&mut field);                         
                            playing = Player::O;
                        }
                        
                        _ => {
                            bots.bot_o.play(&mut field);
                            bots.bot_x.save_move(&mut field);
                            playing = Player::X;
                        }
                    }
                    print(&output, &field);
                    
                },
            }
        }
    }
}
pub mod game;

#[cfg(test)]
mod field_test;