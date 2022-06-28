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
    fn update(&self);
}

// The main engine struct you'll want to instantiate
#[derive(Default)]
pub struct EntityEventEngine {
    time: f64,
    entity_queue: BinaryHeap<EngineTrackedEntity>,
}

impl EntityEventEngine {
    // This trait bound is the secret sauce that allows any TimedEntity object to be added. It must also be static.
    pub fn add_entity<T:TimedEntity + 'static>(&mut self, entity: T) {
        let cooldown = speed_to_time_cooldown(entity.get_speed());
        self.entity_queue.push(EngineTrackedEntity {
            // because we are using dynamic dispatch, entities must be Boxed so EngineTrackedEntities have a fixed size.
            entity: Box::new(entity), // a fixed size box pointing to some unknown size obj.
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
        tracked_entity.entity.update();

        let cooldown = speed_to_time_cooldown(tracked_entity.entity.get_speed());
        tracked_entity.next_update = self.time + cooldown;
        self.entity_queue.push(tracked_entity);
    }
}

// This "container" struct pairs the generic entity with a next_update float,
// so the engine knows when to call update() on the entity.
struct EngineTrackedEntity {
    entity: Box<dyn TimedEntity>, // using dynamic dispatch here so ANY object using TimedEntity can be put in.
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
