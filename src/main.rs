use macroquad::prelude::*;

const GRID_SIZE: f32 = 20.0;

#[macroquad::main("MacroSnake")]
async fn main() {
    let (mut pos_x, mut pos_y) = (0.0, 0.0);

    loop {
        // match get_last_key_pressed() {
        //     Some(KeyCode::H) => pos_x -= GRID_SIZE,
        //     Some(KeyCode::L) => pos_x += GRID_SIZE,
        //     Some(KeyCode::J) => pos_y -= GRID_SIZE,
        //     Some(KeyCode::K) => pos_y += GRID_SIZE,
        //     _ => {}
        // }
        if is_key_down(KeyCode::H) {
            pos_x -= GRID_SIZE;
        } else if is_key_down(KeyCode::L) {
            pos_x += GRID_SIZE;
        } else if is_key_down(KeyCode::J) {
            pos_y -= GRID_SIZE;
        } else if is_key_down(KeyCode::K) {
            pos_y += GRID_SIZE;
        }
        clear_background(BLACK);
        draw_rectangle(pos_x, pos_y, GRID_SIZE, GRID_SIZE, GREEN);
        next_frame().await
    }
}
