use rustbox::{
    Color,
    Style,
    RustBox,
};

#[derive(Clone, Copy)]
pub struct Cell {
    pub ch: char,
    pub style: Style,
    pub fg: Color,
    pub bg: Color,
}

pub fn print_cells(rb: &RustBox, mut x: usize, y: usize, cells: &[Cell]) {
    for cell in cells {
        rb.print_char(x, y, cell.style, cell.fg, cell.bg, cell.ch);
        x += 1;
    }
}

