use crate::bot::memory::Status;

const COST: i32 = 1;
const START_WEIGHT: i32 = 100;

pub struct Node {
    pub status: Status,
    pub movement: usize,
    pub active: bool,
    pub weight: i32,
}

impl Node {
    pub fn new(status: Status, movement: usize) -> Self {
        let weight = START_WEIGHT;
        let active = false;
        Node { status, movement, active, weight }
    }

    pub fn set_end(&mut self, status: &Status) {
        if !self.active {
            panic!("the node is not active");
        }

        self.active = false;
        match status {
            Status::Draw => self.draw(),
            Status::Lose => self.lost(),
            Status::Win => self.won(),
            _ => {}
        }
    }

    fn won(&mut self) {
        self.weight += COST;
    }

    fn lost(&mut self) {
        self.weight -= 2 * COST;
    }

    fn draw(&mut self) {
        self.weight -= COST;
    }
}
