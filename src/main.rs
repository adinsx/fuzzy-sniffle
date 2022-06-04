mod map;

use macroquad::prelude::*;
use map::DungeonMap;

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
    // Max's testing start
    let dungeon = DungeonMap::new(50, 30);
    println!("{}", dungeon.map);
    // Max's testing end

    let suwako_tex: Texture2D = load_texture("suwako.png").await.unwrap();
    let tex_width = suwako_tex.width();
    let tex_height = suwako_tex.height();

    let mut player_x: f32 = 0.0;
    let mut player_y: f32 = 0.0;

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

        next_frame().await
    }
}
