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

pub fn print_cells(rb: &RustBox, x: usize, y: usize, cells: &[Cell]) {
    for (i, cell) in cells.iter().enumerate() {
        rb.print_char(x + i, y, cell.style, cell.fg, cell.bg, cell.ch);
    }
}

pub fn print_cell_repeated_x(rb: &RustBox, x: usize, y: usize, cell: Cell, count: usize) {
    for i in 0..count {
        rb.print_char(x + i, y, cell.style, cell.fg, cell.bg, cell.ch);
    }
}

pub fn print_cell_repeated_y(rb: &RustBox, x: usize, y: usize, cell: Cell, count: usize) {
    for i in 0..count {
        rb.print_char(x, y + i, cell.style, cell.fg, cell.bg, cell.ch);
    }
}
