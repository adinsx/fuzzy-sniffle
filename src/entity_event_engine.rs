use std::{
    cmp::Ordering,
    collections::{
        HashMap,
        BinaryHeap, hash_map::Entry
    },
};

use core::hash::Hash;

/*
Example usage:

let mut entity_engine = EntityEventEngine::default();
// TestEntity and TestEntity2 impl the TimedEntity trait!
entity_engine.add_entity(TestEntity { name: "a".into(), speed: 20.0 });
entity_engine.add_entity(TestEntity2 { name: "b".into(), speed: 100.0, whatever: 1.0 });
for _ in 0..10 {
    entity_engine.update_next();
}
*/

// All game entities that need to be in the EntityEventEngine must implement this trait!
pub trait TimedEntity<K> {
    fn get_speed(&self) -> f64;
    fn update(&self, game_state: &mut HashMap<K, Box<dyn TimedEntity<K>>>) -> bool;
}

// pub trait EntityHolder<K> {
//     fn get_entity_by_id(&mut self, id: i32) -> Option<Box<dyn TimedEntity<K>>>;
//     fn remove_entity_by_id(&mut self, id: i32) -> Option<Box<dyn TimedEntity<K>>>;
//     fn add_entity(&mut self, entity: Box<dyn TimedEntity<K>>);
//     fn get_next_entity_id(&self) -> i32;
// }

// The main engine struct you'll want to instantiate
pub struct EntityEventEngine<K: Hash + Eq> {
    time: f64,
    // game_state: Box<dyn EntityHolder>,
    pub game_state: HashMap<K, Box<dyn TimedEntity<K>>>,
    entity_queue: BinaryHeap<EngineTrackedEntity<K>>,
}

impl<K: Hash + Eq + Copy> EntityEventEngine<K> {
    pub fn new(game_state: HashMap<K, Box<dyn TimedEntity<K>>>) -> EntityEventEngine<K> {
        EntityEventEngine {
            time: 0.0,
            game_state,
            entity_queue: BinaryHeap::default(),
        }
    }

    pub fn add_entity(&mut self, key: K, entity: Box<dyn TimedEntity<K>>) {
        let cooldown = speed_to_time_cooldown(entity.get_speed());
        self.game_state.insert(key, entity);

        self.entity_queue.push(EngineTrackedEntity {
            entity_id: key,
            next_update: self.time + cooldown,
        });
    }

    pub fn update_next(&mut self) {
        let mut tracked_entity = match self.entity_queue.pop() {
            Some(e) => e,
            None => return, // dont do anything if there are no objects in the queue.
        };
        self.time = tracked_entity.next_update;
        println!("Time is {}", self.time);

        let entity = match self.game_state.remove(&tracked_entity.entity_id) {
            Some(entity) => entity,
            None => return,
        };

        let cooldown = speed_to_time_cooldown(entity.get_speed());
        tracked_entity.next_update = self.time + cooldown;

        let alive = entity.update(&mut self.game_state);

        self.game_state.insert(tracked_entity.entity_id, entity);

        if alive {
            self.entity_queue.push(tracked_entity);
        }
    }
}

// This "container" struct pairs the generic entity with a next_update float,
// so the engine knows when to call update() on the entity.
struct EngineTrackedEntity<K> {
    entity_id: K,
    next_update: f64,
}

// ===== required impls for the container struct so they can be put in a BinaryHeap =====
impl<K> PartialOrd for EngineTrackedEntity<K> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.next_update.partial_cmp(&self.next_update)
    }
}

impl<K> Ord for EngineTrackedEntity<K> {
    fn cmp(&self, other: &Self) -> Ordering {
        match other.partial_cmp(self) {
            Some(o) => o,
            None => std::cmp::Ordering::Greater,
        }
    }
}

impl<K> PartialEq for EngineTrackedEntity<K> {
    fn eq(&self, other: &Self) -> bool {
        self.next_update == other.next_update
    }
}

impl<K> Eq for EngineTrackedEntity<K> {}
// ===== END required impls for the container struct so they can be put in a BinaryHeap =====

// just an example function to convert a speed to a time to wait till the next update.
fn speed_to_time_cooldown(speed: f64) -> f64 {
    100.0 / (0.01 * speed + 1.0)
}
