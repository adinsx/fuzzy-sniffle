use crate::entity_event_engine::{TimedEntity};
use crate::game_state::GameState;

#[derive(Debug, Clone, Default)]
pub struct TestEntity {
    pub name: String,
    pub speed: f64,
}

impl TimedEntity<u64> for TestEntity {
    fn get_speed(&self) -> f64 {
        self.speed
    }
    // Box<dyn EntityHolder>
    fn update(&self, game_state: &mut GameState) -> bool {
        println!("Entity {} is updating!", self.name);
        // let entity_to_change = game_state.get(1);

        // wanted to change any value on an entity, but rust has no way of knowing what is availible unless:

        // make a huge trait
        // make 1 god object
        // using any and checking types
        // an ECS system where you can fetch any property for an actor. hashmaps for all props
        // an ECS that handles everything for you (Bevy)

        true
    }
}

#[derive(Debug, Clone, Default)]
pub struct TestEntity2 {
    pub name: String,
    pub speed: f64,
    pub whatever: f64,
}

impl TimedEntity<u64> for TestEntity2 {
    fn get_speed(&self) -> f64 {
        self.speed
    }
    fn update(&self, game_state: &mut GameState) -> bool {
        println!("TestEntity2 {} is updating!", self.name);
        true
    }
}
