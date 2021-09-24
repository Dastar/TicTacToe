use std::env;
use std::process;
use tictactoe::play;

fn parse_arguments(args: Vec<String>) -> play::Config {
    let mut conf = play::Config {
            total_games: 1000000,
            player_first: true
        };
    
    for i in 0..args.len() {
        match args[i].as_str() {
            "help" => {
                println!("help for help");
                println!("total n for total learning cicles");
                println!("second to play for O");
                println!("while playing, ");
                println!("   q for exit");
                println!("   w,e,r,s,d,f,x,c,v for play");
                process::exit(0x0100);
            },
            "total" => {
                if i + 1 >= args.len() {
                    continue;
                }

                conf.total_games = args[i + 1].trim().parse().unwrap_or(1000000);

            },
            "second" => {
                conf.player_first = false;
            },
            _ => {}
        }
    }

    conf
}

fn main() {
    let args: Vec<String> = env::args().collect();
    play::play(parse_arguments(args));

}
