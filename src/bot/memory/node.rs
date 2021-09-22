use crate::bot::memory::Status;

const COST: i32 = 1;
const START_WEIGHT: i32 = 100;

pub struct Node {
    pub status: Status,
    pub movement: usize,
    weight: i32,
}

impl Node {
    pub fn new(status: Status, movement: usize) -> Self {
        let weight = START_WEIGHT;
        Node { status, movement, weight }
    }

    pub fn increse_weight(&mut self) {
        self.weight += COST;
    }

    pub fn decrese_weight(&mut self) {
        self.weight -= COST;
    }
}
