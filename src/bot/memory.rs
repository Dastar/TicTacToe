use crate::game::Player;

const START_WEIGHT: i32 = 100;

enum Status {
    Move(usize),
    StartGame,
    Win,
    Lose,
    Draw
}

enum Node {
    Memory(Box<Memory>),
    None
}

impl Default for Node {
    fn default() -> Self {
        Node::None
    }
}

impl Node {
    fn new(my_move: bool, player: Player, status: Status) -> Node {
        Node::Memory(Box::new(Memory::new(my_move, player, status)))
    }
}


struct Memory {
    status: Status,
    moves: [Node; 9],
    active: bool,
    weight: i32,
    my_move: bool,
    play: Player
}


impl Memory {
    fn new(my_move: bool, player: Player, status: Status) -> Memory { 
        Memory {
            status: status,
            moves: Default::default(),
            active: false,
            weight: START_WEIGHT,
            my_move: my_move,
            play: player,
        }
    }

    fn get_next_move(&mut self) {
        match self.status {
            Status::Move(i) => println!("foo"),
            Status::StartGame => println!("bar"),
            Status::Win => println!("bar"),
            Status::Lose => println!("bar"),
            Status::Draw => println!("bar"),
        };
    }
 
    fn find_next_move(&mut self) {
        match self.moves[0] {
            Node::None => {
                self.create_moves(!self.my_move);
            }
            _ => {}
        }

        
    }

    fn create_moves(&mut self, my_move: bool) {
        let player = if my_move { self.play } else { Player::oponent(&self.play)  };
        for i in 0..9 {
            self.moves[i] = Node::new(my_move, player, Status::Move(i));
        }
    }
}