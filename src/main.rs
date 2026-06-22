use macroquad::prelude::*;

#[macroquad::main("Cat")]
async fn main() {
    let texture: Texture2D = load_texture("assets/default-cat-sprite.png").await.unwrap();
    texture.set_filter(FilterMode::Nearest);

    let frame_width = 32.0;
    let frame_height = 32.0;
    let total_frames = 8;

    let mut current_frame = 0;
    let mut frame_timer = 0.0;
    let frame_duration = 0.1;

    loop {
        clear_background(Color::new(0.0, 0.0, 0.0, 0.0));

        frame_timer += get_frame_time();
        if frame_timer >= frame_duration {
            frame_timer = 0.0;
            current_frame = (current_frame + 1) % total_frames;
        }

        // which part of the sprite sheet to draw
        let source_rect = Rect::new(
            current_frame as f32 * frame_width,
            35.0, // first row
            frame_width,
            frame_height,
        );

        draw_texture_ex(
            &texture,
            100.0,
            100.0,
            WHITE,
            DrawTextureParams {
                source: Some(source_rect),
                dest_size: Some(vec2(128.0, 128.0)),
                ..Default::default()
            },
        );

        next_frame().await
    }
}
