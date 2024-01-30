use piston_window as pw;

const BLOCK_SIZE: f64 = 25.0;

pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

pub fn to_coord_u32(game_coord: i32) -> u32 {
    to_coord(game_coord) as u32
}

pub fn draw_rectangle(
    color: pw::types::Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    con: &pw::Context,
    g: &mut pw::G2d,
) {
    pw::rectangle(
        color,
        [
            to_coord(x),
            to_coord(y),
            BLOCK_SIZE * (width as f64),
            BLOCK_SIZE * (height as f64),
        ],
        con.transform,
        g,
    )
}

pub fn draw_block(color: pw::types::Color, x: i32, y: i32, con: &pw::Context, g: &mut pw::G2d) {
    draw_rectangle(color, x, y, 1, 1, con, g);
}
