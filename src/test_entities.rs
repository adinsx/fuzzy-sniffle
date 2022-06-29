// mod entity_event_engine;
use crate::entity_event_engine::{EntityHolder, TimedEntity};
use crate::game_state::GameState;

#[derive(Debug, Clone, Default)]
pub struct TestEntity {
    pub name: String,
    pub speed: f64,
}

impl TimedEntity for TestEntity {
    fn get_speed(&self) -> f64 {
        self.speed
    }
    // Box<dyn EntityHolder>
    fn update(&self, game_state: &mut dyn EntityHolder) -> bool {
        println!("Entity {} is updating!", self.name);
        true
    }
}

#[derive(Debug, Clone, Default)]
pub struct TestEntity2 {
    pub name: String,
    pub speed: f64,
    pub whatever: f64,
}

impl TimedEntity for TestEntity2 {
    fn get_speed(&self) -> f64 {
        self.speed
    }
    fn update(&self, game_state: &mut dyn EntityHolder) -> bool {
        println!("TestEntity2 {} is updating!", self.name);
        true
    }
}
