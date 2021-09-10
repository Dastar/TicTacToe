use std::io;

use tictactoe::game::*;

fn main() {
    let mut field = Field::new();
    let mut playing = Player::X;

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

                playing = match playing {
                    Player::X => Player::O,
                    _ => Player::X
                }

            },
        }
    }

}
