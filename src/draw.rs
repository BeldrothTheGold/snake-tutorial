use piston_window::{rectangle, types::Color, Context, G2d};
const BLOCK_SIZE: u32 = 40;

#[derive(PartialEq, Copy, Clone)]
pub struct Block {
    pub x: u32,
    pub y: u32,
}

impl Block {
    pub fn new(x: u32, y: u32) -> Block {
        Block { x, y }
    }
    pub fn to_coords(&self) -> (f64, f64) {
        crate::draw::to_coord(self.x, self.y)
    }
    pub fn draw(&self, color: Color, con: &Context, g: &mut G2d) {
        let (gui_x, gui_y) = self.to_coords();
        rectangle(
            color,
            [gui_x, gui_y, BLOCK_SIZE as f64, BLOCK_SIZE as f64],
            con.transform,
            g,
        );
    }
}

pub fn to_coord(x: u32, y: u32) -> (f64, f64) {
    ((x * BLOCK_SIZE) as f64, (y * BLOCK_SIZE) as f64)
}

pub struct Rectangle {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Rectangle {
        Rectangle {
            x,
            y,
            width,
            height,
        }
    }

    pub fn draw(&self, color: Color, con: &Context, g: &mut G2d) {
        let (gui_x, gui_y) = to_coord(self.x, self.y);
        let (gui_w, gui_h) = to_coord(self.width, self.height);
        rectangle(color, [gui_x, gui_y, gui_w, gui_h], con.transform, g);
    }
    pub fn contains(&self, block: &Block) -> bool {
        if block.x < self.x
            || block.x > self.x + self.width -1
            || block.y < self.y
            || block.y > self.y + self.height -1
        {
            false
        } else {
            true
        }
    }
}
