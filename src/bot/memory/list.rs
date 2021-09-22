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

impl List {
    pub fn new() -> Self {
        let mut list: Vec<Next> = Vec::new();
        let mut moves: Vec<usize> = Vec::new();

        for movement in 0..9 {
            list.push(List::get_elem(movement));
            moves.push(movement)
        }

        List { list, moves }
    }

    fn add_nodes(&mut self) -> Result<(), &str> {
        if self.list.len() == 1 {
            return Err("cannot add new nodes");
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

                list.push(List::get_elem(movement));
                moves.push(movement)

            }

            element.link = Link::Next(Box::new(List{ list, moves }))
        }

        Ok(())
    }

    fn get_elem(movement: usize) -> Next {
        Next { node: Node::new(Status::Progress, movement), link: Link::Empty }
    }

}

#[cfg(test)]
mod tests_list {
    use crate::bot::memory::list::List;
    use crate::bot::memory::list::Link;

    #[test]
    fn create_list() {
        let mut list = List::new();
        assert_eq!(list.moves.len(), 9);

        assert_eq!(list.add_nodes(), Ok(()));
        for n in list.list {
            match n.link {
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
    }
}
