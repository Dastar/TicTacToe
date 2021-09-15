use crate::game::Player;

const START_WEIGHT: i32 = 100;
const COST: i32 = 1;

enum Status {
    Move(usize),
    StartGame,
    Played,
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

    fn value(&self) -> &Memory {
        match self {
            Node::Memory(m) => m,
            Node::None => panic!("Cannot get a value")
        }
    }

    fn as_mut(&mut self) -> &mut Memory {
        match self {
            Node::Memory(m) => m,
            Node::None => panic!("Cannot get a value")
        }
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
    pub fn new(my_move: bool, player: Player, status: Status) -> Memory { 
        Memory {
            status: status,
            moves: Default::default(),
            active: false,
            weight: START_WEIGHT,
            my_move: my_move,
            play: player,
        }
    }

    pub fn end_game(&mut self, winner: Player) {
        let status = self.get_final_status(winner);
        self.set_end_status(status);
    }

    pub fn set_opponent_move(&mut self, point: usize) {

    }

    fn set_end_status(&mut self, status: Status) {
        self.active = false;
        match status {
            Status::Win => self.weight += COST,
            Status::Lose => self.weight -= 2 * COST,
            Status::Draw => self.weight -= COST,
            _ => panic!("This status is not correct")
        }

        match self.moves[0] {
            Node::None => self.status = status,
            _ => {
                let active: &mut Memory = self.find_active().unwrap();
                active.set_end_status(status);
            }
        }
    }

    fn get_final_status(&self, winner: Player) -> Status {
        match winner {
            Player::None => Status::Draw,
            _ => {
                if winner == self.play && self.my_move || 
                   winner != self.play && !self.my_move {
                       Status::Win
                   }
                   else {
                       Status::Lose
                   }
            }
        }
    }

    pub fn get_next_move(&mut self) {
        self.active = true;


        match self.status {
            Status::Move(i) => println!("foo"),
            Status::StartGame => println!("bar"),
            _ => panic!("This is the last node"),
        };
    }
 
    fn find_next_move(&mut self) -> Status{
        match self.moves[0] {
            Node::None => {
                self.create_moves(!self.my_move);
            }
            _ => {}
        }

        self.active = true;
        

        Status::StartGame
    }

    fn find_node(&self) -> &Memory {
        let mut max: &Memory = &self.moves[0].value();
        for m in self.moves.iter() {
            if m.value().active {
                return m.value();
            }
            else if m.value().weight > max.weight {
                max = m.value();
            }
        }

        max
    }

    fn find_active(&mut self) -> Result<&mut Memory, String> {
        for m in self.moves.iter_mut() {
            if m.value().active {
                return Ok(m.as_mut());
            }
        }

        Err("cant find active value".to_string())
    }

    fn create_moves(&mut self, my_move: bool) {
        let player = if my_move { self.play } else { Player::oponent(&self.play)  };
        for i in 0..9 {
            self.moves[i] = Node::new(my_move, player, Status::Move(i));
        }
    }
}