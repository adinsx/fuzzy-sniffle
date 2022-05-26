use macroquad::prelude::*;

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
    let suwakoTex: Texture2D = load_texture("suwako.png").await.unwrap();
    let texWidth = suwakoTex.width();
    let texHeight = suwakoTex.height();

    let mut playerX: f32 = 0.0;
    let mut playerY: f32 = 0.0;

    loop {
        clear_background(LIGHTGRAY);

        if is_key_pressed(KeyCode::Right) {
            playerX += texWidth;
        }
        if is_key_pressed(KeyCode::Left) {
            playerX -= texWidth;
        }
        if is_key_pressed(KeyCode::Down) {
            playerY += texHeight;
        }
        if is_key_pressed(KeyCode::Up) {
            playerY -= texHeight;
        }

        draw_texture(suwakoTex, playerX, playerY, WHITE);

        next_frame().await
    }
}
