use std::f64::consts::E;
use crate::bot::memory::Status;

const COST: f64 = 0.1;
const START_WEIGHT: f64 = 1.0;

pub struct Node {
    pub status: Status,
    pub movement: usize,
    pub active: bool,
    pub weight: f64,
}

impl Node {
    pub fn new(status: Status, movement: usize) -> Self {
        let weight = START_WEIGHT;
        let active = false;
        Node { status, movement, active, weight }
    }

    pub fn set_end(&mut self, status: &Status, turns: i32) {
        if !self.active {
            panic!("the node is not active");
        }

        self.active = false;
        match status {
            Status::Draw => self.draw(),
            Status::Lose => self.lost(turns),
            Status::Win => self.won(turns),
            _ => {}
        }
    }

    pub fn weight(&self) -> f64 {
        if self.weight > 0.0 {
            return self.weight.clone();
        }

        E.powf(self.weight)
    }

    fn won(&mut self, turns: i32) {
        self.weight += COST * 9.0 - 1.0 / (turns as f64);
    }

    fn lost(&mut self, turns: i32) {
        self.weight -= COST * (turns as f64);
    }

    fn draw(&mut self) {
        self.weight += COST * 2.0;
    }
}
