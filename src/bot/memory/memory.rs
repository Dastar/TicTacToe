use crate::game::Player;
//use crate::game::Field;
use crate::bot::memory::list::List;
//use crate::bot::memory::node::Node;

struct Memory {
    player: Player,
    long_memory: List,
}

// impl<'a> Memory<'a> {

//     pub fn create_table(&mut self, field: &Field, player: Player) {
//         for i in 0..9 {
//             if field.can_play(i) {
//                // self.moves.push(Box::new(Memory::new(!self.my_move, player, Status::Move(i))));
//             }
//         }
//     }

//     pub fn get_next_move(&mut self) {
//         let mut next_move = self.find_active();
//         match next_move {
//             Some(m) => {

//             },
//             None => {
                
//             }
//         }
//     }

//     fn find_active(&mut self) -> Option<&mut Memory> {
//         // for m in self.moves.iter_mut() {
//         //     if m.active {
//         //         return Some(m.as_mut());
//         //     }
//         // }

//         None
//     }

//     //fn find_weighted(&mut self) -> &mut Memory {
//         //self.moves.iter_mut().max_by_key(|m| m.weight).unwrap()

//     //}
// }