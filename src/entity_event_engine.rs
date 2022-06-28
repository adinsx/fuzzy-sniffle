use std::{
    collections::BinaryHeap,
    cmp::Ordering
};

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
pub trait TimedEntity {
    fn get_speed(&self) -> f64;
    fn update(&self, game_state: &mut dyn EntityHolder) -> bool;
}

pub trait EntityHolder {
    fn get_entity_by_id(&self, id: i32) -> Option<&dyn TimedEntity>;
    fn remove_entity_by_id(&mut self, id: i32) -> Option<&dyn TimedEntity>;
    fn add_entity(&mut self, entity: Box<dyn TimedEntity>);
    fn get_next_entity_id(&self) -> i32;
}

// The main engine struct you'll want to instantiate
pub struct EntityEventEngine {
    time: f64,
    game_state: Box<dyn EntityHolder>,
    entity_queue: BinaryHeap<EngineTrackedEntity>,
}

impl EntityEventEngine {
    pub fn new(game_state: Box<dyn EntityHolder>) -> EntityEventEngine {
        EntityEventEngine {
            time: 0.0,
            game_state,
            entity_queue: BinaryHeap::default()
        }
    }

    // This trait bound is the secret sauce that allows any TimedEntity object to be added. It must also be static.
    // pub fn add_entity<T:TimedEntity + 'static>(&mut self, entity: T) {
    //     let cooldown = speed_to_time_cooldown(entity.get_speed());
    //     self.entity_queue.push(EngineTrackedEntity {
    //         // because we are using dynamic dispatch, entities must be Boxed so EngineTrackedEntities have a fixed size.
    //         entity: Box::new(entity), // a fixed size box pointing to some unknown size obj.
    //         next_update: self.time + cooldown
    //     });
    // }
    // pub fn add_entity(&mut self, entity_id: i32) {
    //     let entity = match self.game_state.get_entity_by_id(entity_id) {
    //         Some(entity) => entity,
    //         None => return
    //     };
    //     let cooldown = speed_to_time_cooldown(entity.get_speed());
    //     self.entity_queue.push(EngineTrackedEntity {
    //         entity_id,
    //         next_update: self.time + cooldown
    //     });
    // }
    pub fn add_entity(&mut self, entity: Box<dyn TimedEntity>) {
        let entity_id = self.game_state.get_next_entity_id();
        let cooldown = speed_to_time_cooldown(entity.get_speed());
        self.game_state.add_entity(entity);

        self.entity_queue.push(EngineTrackedEntity {
            entity_id,
            next_update: self.time + cooldown
        });
    }

    pub fn update_next(&mut self) {
        let mut tracked_entity = match self.entity_queue.pop() {
            Some(e) => e,
            None => return // dont do anything if there are no objects in the queue.
        };
        self.time = tracked_entity.next_update;
        println!("Time is {}", self.time);

        let entity = match self.game_state.get_entity_by_id(tracked_entity.entity_id) {
            Some(entity) => entity,
            None => return
        };

        let alive = entity.update(self.game_state.as_mut());

        let cooldown = speed_to_time_cooldown(entity.get_speed());
        tracked_entity.next_update = self.time + cooldown;

        if alive {
            self.entity_queue.push(tracked_entity);
        }
    }
}

// This "container" struct pairs the generic entity with a next_update float,
// so the engine knows when to call update() on the entity.
struct EngineTrackedEntity {
    entity_id: i32,
    next_update: f64
}

// ===== required impls for the container struct so they can be put in a BinaryHeap =====
impl PartialOrd for EngineTrackedEntity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.next_update.partial_cmp(&self.next_update)
    }
}

impl Ord for EngineTrackedEntity {
    fn cmp(&self, other: &Self) -> Ordering {
        match other.partial_cmp(&self) {
            Some(o) => o,
            None => std::cmp::Ordering::Greater,
        }
    }
}

impl PartialEq for EngineTrackedEntity {
    fn eq(&self, other: &Self) -> bool {
        self.next_update == other.next_update
    }
}

impl Eq for EngineTrackedEntity {}
// ===== END required impls for the container struct so they can be put in a BinaryHeap =====

// just an example function to convert a speed to a time to wait till the next update.
fn speed_to_time_cooldown(speed: f64) -> f64 {
    100.0 / (0.01 * speed + 1.0)
}
