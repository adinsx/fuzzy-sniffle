use std::collections::HashMap;

use crate::entity_event_engine::{TimedEntity};

// #[derive(Default)]
// pub struct GameState {
//     pub entities: HashMap<u64, Box<dyn TimedEntity<u64>>>,
// }

pub type GameState = HashMap<u64, Box<dyn TimedEntity<u64>>>;

// impl EntityHolder for GameState {
//     fn get_entity_by_id(&mut self, entity_id: u64) -> Option<Box<dyn TimedEntity>> {
//         self.entities.remove(&entity_id)
//     }

//     fn remove_entity_by_id(&mut self, entity_id: u64) -> Option<Box<dyn TimedEntity>> {
//         self.entities.remove(&entity_id)
//         /*         let b = match a {
//             Some(a) => a,
//             None => panic!("FUCK"),
//         };
//         return Some(b); */
//     }

//     fn get_next_entity_id(&self) -> u64 {
//         self.entities.len() as u64
//     }

//     fn add_entity(&mut self, entity: Box<dyn TimedEntity>) {
//         self.entities.insert(self.entities.len() as u64, entity);
//     }
// }
