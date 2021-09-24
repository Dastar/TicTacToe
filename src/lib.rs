pub mod bot;

pub mod play {
    use super::game::*;
    use crate::bot::play_bot;
    use std::io;

    pub struct Config {
        pub total_games: usize,
        pub player_first: bool
    }
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
    
    pub fn play(conf: Config) {
        let mut bots = Bots::new();
        play_bots(&mut bots, conf.total_games);
        
        let choosen_bot = {
            if conf.player_first {
                &mut bots.bot_o
            }
            else {
                &mut bots.bot_x
            }
        };
        
        play_game(choosen_bot);
    }

    fn parse(c: char) -> usize {
        match c {
            'w' => 1,
            'e' => 2,
            'r' => 3,
            's' => 4,
            'd' => 5,
            'f' => 6,
            'x' => 7,
            'c' => 8,
            'v' => 9,
            _ => 1,
        }
    }

    fn play_game(computer: &mut play_bot::Bot) {
        let mut field = Field::new();
        let mut playing = Player::X;
        
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
                    if playing == computer.player {
                        computer.play(&mut field);
                    } 
                    else {
                        match human_play(&mut field, &playing) {
                            HStatus::Err => continue,
                            HStatus::Exit => break,
                            _ => {}
                        }        
                        computer.save_move(&mut field);
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

    #[derive(PartialEq)]
    enum HStatus {
        Ok,
        Err,
        Exit
    }
    fn human_play(field: &mut Field, playing: &Player) -> HStatus {
        println!("Enter point:");
        let mut point = String::new();
       
        io::stdin()
        .read_line(&mut point)
        .expect("Failed to read line");
       
        let point: char = point.trim().parse().unwrap_or('w');
        if point == 'q' {
            return HStatus::Exit;
        }
       
        if let Err(_) = field.one_play(&playing, parse(point) - 1) {
            println!("Cant go here");
            return HStatus::Err;
        }
        HStatus::Ok
    }

    fn print(output: &bool, field: &Field) {
        if *output {
            println!("{}", field);
        }
    }

    fn play_bots(bots: &mut Bots, total_cycles: usize) {
        let mut field = Field::new();
        let mut playing = Player::X;
        let mut total_games: usize = 0;
        let output = false;
        loop {
            match field.game_progress() {
                GameStatus::Win(player) => {
                    total_games += 1;
                    if total_games % (total_cycles / 10 as usize) == 0 {
                        bots.bot_x.statistics(total_games);
                    }

                    bots.bot_x.end(player);
                    bots.bot_o.end(player);
                    playing = Player::X;
                    field = Field::new();

                    if total_games == total_cycles {
                        break;
                    }
                },
                GameStatus::Play => {
                    match playing {
                        Player::X => {
                            bots.bot_x.learn(&mut field);   
                            bots.bot_o.save_move(&mut field);                         
                            playing = Player::O;
                        }
                        
                        _ => {
                            bots.bot_o.learn(&mut field);
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