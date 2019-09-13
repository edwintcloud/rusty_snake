use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d};

/// BLOCK_SIZE is the of the block.
const BLOCK_SIZE: f64 = 25.0;

/// Scale a {game_cord} by the block size.
pub fn to_coord(game_cord: i32) -> f64 {
    (game_cord as f64) * BLOCK_SIZE
}

/// Scale a {game_cord} by the block size.
pub fn to_coord_u32(game_cord: i32) -> u32 {
    to_coord(game_cord) as u32
}

/// Draw a rectangle (square) of BLOCK_SIZE x BLOCK_SIZE.
pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    // add rectangle to the graphics buffer
    rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE], // posX, posY, width, height
        con.transform,                          // context transform ref
        g,                                      // graphics buffer mut ref
    );
}

/// Draw a rectangle of width x height.
pub fn draw_rectangle(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d,
) {
    let x = to_coord(x);
    let y = to_coord(y);

    // add rectangle to the graphics buffer
    rectangle(
        color,
        [
            x,                            // posX
            y,                            // posY
            BLOCK_SIZE * (width as f64),  // width
            BLOCK_SIZE * (height as f64), // height
        ],
        con.transform, // context transform ref
        g,             // graphics buffer mut ref
    );
}
