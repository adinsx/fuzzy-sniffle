use std::{
    collections::BinaryHeap,
    cmp::Ordering
};

pub trait TimedEntity {
    fn get_speed(&self) -> f32;
    fn update(&self);
}

#[derive(Debug, Copy, Clone)]
struct EngineTrackedEntity<T: TimedEntity> {
    entity: T,
    next_update: f32
}

impl<T: TimedEntity> PartialOrd for EngineTrackedEntity<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.next_update.partial_cmp(&self.next_update)
    }
}

impl<T: TimedEntity> Ord for EngineTrackedEntity<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        match other.next_update.partial_cmp(&self.next_update) {
            Some(o) => o,
            None => std::cmp::Ordering::Greater,
        }
    }
}

impl<T: TimedEntity> PartialEq for EngineTrackedEntity<T> {
    fn eq(&self, other: &Self) -> bool {
        self.next_update == other.next_update
    }
}

impl<T: TimedEntity> Eq for EngineTrackedEntity<T> {}

#[derive(Debug, Default, Clone)]
pub struct EntityEventEngine<T: TimedEntity> {
    time: f32,
    entity_queue: BinaryHeap<EngineTrackedEntity<T>>,
}

impl<T: TimedEntity> EntityEventEngine<T> {
    pub fn add_entity(&mut self, entity: T) {
        let cooldown = speed_to_time_cooldown(entity.get_speed());
        self.entity_queue.push(EngineTrackedEntity {
            entity: entity,
            next_update: self.time + cooldown
        });
    }

    pub fn update_next(&mut self) {
        let mut tracked_entity = self.entity_queue.pop().unwrap();
        self.time = tracked_entity.next_update;
        println!("Time is {}", self.time);
        tracked_entity.entity.update();

        let cooldown = speed_to_time_cooldown(tracked_entity.entity.get_speed());
        tracked_entity.next_update = self.time + cooldown;
        self.entity_queue.push(tracked_entity);
    }
}

fn speed_to_time_cooldown(speed: f32) -> f32 {
    100.0 / (0.01 * speed + 1.0)
}
