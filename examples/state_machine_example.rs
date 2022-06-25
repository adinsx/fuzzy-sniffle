#[path = "../src/state_machine.rs"]
mod state_machine;

use state_machine::{DelayedAction, StateMachine};

fn generate_events<'a>(state: &i32) -> Vec<DelayedAction<'a, i32>> {
    match state {
        3 => vec![DelayedAction::new(|x| x + 4, 3.0)],
        7 => vec![
            DelayedAction::new(|x| x - 1, 13.0),
            DelayedAction::new(|x| x + 20, 12.0),
        ],
        6 => vec![DelayedAction::new(|x| x + 9, 0.1)],
        27 => vec![DelayedAction::new(|x| x - 21, 4.0)],
        _ => Vec::new(),
    }
}

fn main() {
    let world = StateMachine::new(
        3,
        |x| {
            println!("{x}");
            generate_events(x)
        },
        f32::MIN,
    );
    world.run();
}
