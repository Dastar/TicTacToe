use std::cmp::Ordering;
use rand::prelude::*;
use crate::bot::memory::node::Node;
use crate::bot::memory::Status;

enum Link {
    Empty,
    Next(Box<List>)
}

pub struct List {
    list: Vec<Next>,
    moves: Vec<usize>
}

struct Next {
    node: Node,
    link: Link,
}

#[derive(PartialEq,PartialOrd)]
struct Float(f64);

impl Float {
    fn new(val: f64) -> Option<Self> {
        if val.is_nan() {
            None
        } else {
            Some(Float(val))
        }
    }
}

impl Eq for Float {}

impl Ord for Float {
    fn cmp(&self, other: &Float) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

/// For now this implementation take some assumptions that one cannot play twice the same move
impl List {
    pub fn new() -> Self {
        let mut list: Vec<Next> = Vec::new();
        let mut moves: Vec<usize> = Vec::new();

        for movement in 0..9 {
            list.push(List::get_next_elem(movement));
            moves.push(movement)
        }

        List { list, moves }
    }

    pub fn play_move(&mut self) -> usize {
        self.get_move(|l| l.iter_mut()
                                                  .max_by_key(|m| Float::new(m.node.weight()))
                                                  .unwrap()
                    )
    }

    pub fn learn_move(&mut self) -> usize {
        self.get_move(|l| {
            let mut rng = thread_rng();
            let res = l.choose_weighted_mut(&mut rng, |next| next.node.weight());
            match res {
                Ok(n) => n,
                Err(e) => panic!("{}", e),
            }
        })
    }

    pub fn get_move<F>(&mut self, take_node: F) -> usize where
    F: Fn(&mut Vec<Next>) -> &mut Next {
        self.add_nodes().unwrap_or(());

        // first we search for an active node
        // if found we go to next line
        // else take node with greatest weight
        let active = self.list.iter_mut().find(|next| next.node.active);
        match active {
            Some(next) => {
                match &mut next.link {
                    Link::Empty => next.node.movement.clone(),
                    Link::Next(link) => link.get_move(take_node),
                }
            },
            None => {
                let n = take_node(&mut self.list);
                n.node.active = true;
                n.node.movement.clone()

            }
        }
    }

    pub fn set_move(&mut self, movement: usize) {
        let active = self.list.iter_mut().find(|next| next.node.active);
        match active {
            Some(next) => {
                if let Link::Next(link) = &mut next.link {
                    link.set_move(movement);
                }
            },
            None => {
                for next in self.list.iter_mut() {
                    if next.node.movement == movement {
                        next.node.active = true;
                    }
                }
            }
        }
    }

    pub fn end_game(&mut self, status: &Status, turns: i32) {
        let active = self.list.iter_mut().find(|next| next.node.active);
        match active {
            Some(next) => {
                match &mut next.link {
                    Link::Next(link) => {
                        // we need to search for next active to make sure that game not ends here
                        let sub_active = link.list.iter_mut().find(|next| next.node.active);
                        match sub_active {
                            None => next.node.status = status.clone(),
                            Some(_) => link.end_game(status, turns),
                        }
                    }
                    Link::Empty => next.node.status = status.clone(),
                }

                next.node.set_end(status, turns);
            },

            None => {
                let last = self.list.iter_mut().find(|next| &next.node.status == status);
                match last {
                    Some(next) => {
                        next.node.set_end(status, turns);
                        next.link = Link::Empty;
                    },
                    None => panic!("No last turn active"),
                }
            },
        }
    }

    fn add_nodes(&mut self) -> Result<(), &str> {
        if self.list.len() == 1 {
            return Err("cannot add new nodes");
        }

        if let Link::Next(_) = self.list[0].link {
            return Err("there is a next line already");
        }

        // creating next line of nodes
        for element in self.list.iter_mut() {
            let mut list: Vec<Next> = Vec::new();
            let mut moves: Vec<usize> = Vec::new();

            for m in self.moves.iter() {
                let movement = m.clone();
                
                // move in next line cannot be in the same place, as in the previous node
                if element.node.movement == movement {
                    continue;
                }

                // no need to add a row if this node is last
                if let Status::Progress = element.node.status {
                    list.push(List::get_next_elem(movement));
                    moves.push(movement)
                }

            }

            element.link = Link::Next(Box::new(List{ list, moves }))
        }

        Ok(())
    }

    fn get_next_elem(movement: usize) -> Next {
        Next { node: Node::new(Status::Progress, movement), link: Link::Empty }
    }

}

#[cfg(test)]
mod tests_list {
    use crate::bot::memory::list::List;
    use crate::bot::memory::list::Link;
    use crate::bot::memory::Status;
    #[test]
    fn create_list() {
        let mut list = List::new();
        assert_eq!(list.moves.len(), 9);

        assert_eq!(list.add_nodes(), Ok(()));
        for n in &list.list {
            match &n.link {
                Link::Empty => assert!(false),
                Link::Next(l) => {
                    assert_eq!(l.moves.len(), 8);
                    assert_eq!(l.list.len(), 8);
                    for next in l.list.iter() {
                        assert_ne!(next.node.movement, n.node.movement);
                    }
                }
            }
            
        }

        assert_eq!(list.add_nodes(), Err("there is a next line already"));
    }

    #[test]
    fn play() {
        // let mut list = List::new();
        // let m = list.get_move();
        // assert_eq!(list.get_move(), 8);
        // list.set_move(7);
        // assert_eq!(list.get_move(), 6);
        // list.set_move(4);
        // assert_eq!(list.get_move(), 5);
        // list.set_move(3);
        // assert_eq!(list.get_move(), 2);
        // list.set_move(1);
        // assert_eq!(list.get_move(), 0);
        // assert_eq!(list.get_move(), 0);
        // list.end_game(&Status::Win);

        // assert_eq!(list.get_move(), 8);
        // list.set_move(4);
        // assert_eq!(list.get_move(), 7);
        // list.set_move(6);
        // assert_eq!(list.get_move(), 5);
        // list.set_move(3);
        // assert_eq!(list.get_move(), 2);
        // list.set_move(0);
        // assert_eq!(list.get_move(), 1);
        // list.end_game(&Status::Lose);
    }

    #[test]
    fn partial_game() {
        // let mut list = List::new();
        // assert_eq!(list.get_move(), 8);
        // list.set_move(4);
        // assert_eq!(list.get_move(), 7);
        // list.set_move(6);
        // assert_eq!(list.get_move(), 5);
        // list.set_move(3);
        // assert_eq!(list.get_move(), 2);
        // list.end_game(&Status::Win);
    }
}
