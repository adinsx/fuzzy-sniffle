use std::collections::HashMap;

use crate::entity_event_engine::{TimedEntity, EntityHolder};

#[derive(Default)]
pub struct GameState {
    pub entities: HashMap<i32, Box<dyn TimedEntity>>,
}

impl EntityHolder for GameState {
    fn get_entity_by_id(&self, entity_id:i32) -> Option<&dyn TimedEntity> {
        let a = self.entities.get(&entity_id);
        a.map(Box::as_ref)
    }

    fn remove_entity_by_id(&mut self, entity_id:i32) -> Option<&dyn TimedEntity> {
        let a = self.entities.remove(&entity_id);
        let b = match a {
            Some(a) => a,
            None => panic!("FUCK")
        };
        return Some(&*b);
    }

    fn get_next_entity_id(&self) -> i32 {
        self.entities.len() as i32
    }

    fn add_entity(&mut self, entity: Box<dyn TimedEntity>) {
        self.entities.insert(self.entities.len() as i32, entity);
    }
}
