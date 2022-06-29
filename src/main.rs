use macroquad::prelude::*;

mod entity_event_engine;
mod game_state;
mod test_entities;

use game_state::GameState;

use entity_event_engine::{EntityEventEngine, TimedEntity};

use test_entities::{TestEntity, TestEntity2};

fn window_conf() -> Conf {
    Conf {
        window_title: "SimpleGame".into(),
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let suwako_tex: Texture2D = load_texture("suwako.png").await.unwrap();
    let tex_width = suwako_tex.width();
    let tex_height = suwako_tex.height();

    let mut player_x: f32 = 0.0;
    let mut player_y: f32 = 0.0;

    let game_state = Box::new(GameState::default());
    let mut entity_engine = EntityEventEngine::new(game_state);

    {
        entity_engine.add_entity(Box::new(TestEntity {
            name: "a".into(),
            speed: 20.0,
        }));
        entity_engine.add_entity(Box::new(TestEntity2 {
            name: "b".into(),
            speed: 100.0,
            whatever: 1.0,
        }));
    }
    println!("sizof box: {}", std::mem::size_of::<Box<dyn TimedEntity>>());

    for _ in 0..10 {
        entity_engine.update_next();
    }

    loop {
        clear_background(LIGHTGRAY);

        if is_key_pressed(KeyCode::Right) {
            player_x += tex_width;
        }
        if is_key_pressed(KeyCode::Left) {
            player_x -= tex_width;
        }
        if is_key_pressed(KeyCode::Down) {
            player_y += tex_height;
        }
        if is_key_pressed(KeyCode::Up) {
            player_y -= tex_height;
        }

        draw_texture(suwako_tex, player_x, player_y, WHITE);

        next_frame().await;
    }
}
