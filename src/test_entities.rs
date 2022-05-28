// mod entity_event_engine;
use crate::entity_event_engine::TimedEntity;

#[derive(Debug, Clone, Default)]
pub struct TestEntity {
    pub name: String,
    pub speed: f32
}

impl TimedEntity for TestEntity {
    fn get_speed(&self) -> f32 {
        self.speed
    }
    fn update(&self) {
        println!("Entity {} is updating!", self.name);
    }
}

#[derive(Debug, Clone, Default)]
pub struct TestEntity2 {
    pub name: String,
    pub speed: f32,
    pub whatever: f32
}

impl TimedEntity for TestEntity2 {
    fn get_speed(&self) -> f32 {
        self.speed
    }
    fn update(&self) {
        println!("TestEntity2 {} is updating!", self.name);
    }
}
