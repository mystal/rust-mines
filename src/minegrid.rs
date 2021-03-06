use std::collections::HashSet;

use rand::{self, Rng};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellState {
    Hidden(u8),
    Revealed,
}

#[derive(Clone)]
pub struct Cell {
    x: u32,
    y: u32,
    mines: u8,
    state: CellState,
    surrounding_mines: u8,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GridState {
    Play,
    Win,
    Lose,
}

pub struct MineGrid {
    cells: Vec<Vec<Cell>>, // TODO: Use a single vector that you index into.
    width: u32,
    height: u32,
    mines: u32,
    max_mines: u8,
    mines_flagged: u32,
    spaces_left: u32,
    state: GridState,
    //seed: u64,
}

impl Cell {
    pub fn mines(&self) -> u8 {
        self.mines
    }

    pub fn state(&self) -> CellState {
        self.state
    }

    pub fn surrounding_mines(&self) -> u8 {
        self.surrounding_mines
    }
}

impl MineGrid {
    pub fn new(width: u32, height: u32, mines: u32) -> MineGrid {
        let mut cells = Vec::with_capacity(height as usize);

        // Randomly place mines
        let mut rng = rand::thread_rng();
        let mut mine_points = HashSet::new();
        while mine_points.len() != mines as usize {
            let point = (rng.gen_range(0, width),
                         rng.gen_range(0, height));
            mine_points.insert(point);
        }

        for j in 0..height {
            let mut row = Vec::with_capacity(width as usize);
            for i in 0..width {
                row.push(Cell {
                    x: i,
                    y: j,
                    mines: if mine_points.contains(&(i, j)) { 1 } else { 0 },
                    state: CellState::Hidden(0),
                    surrounding_mines: 0,
                });
            }
            cells.push(row);
        }

        let mut grid = MineGrid {
            cells: cells,
            width: width,
            height: height,
            mines: mines,
            max_mines: 1,
            mines_flagged: 0,
            spaces_left: width * height - mines,
            state: GridState::Play,
        };

        // Cache surrounding mine count in each cell.
        for j in 0..height {
            for i in 0..width {
                grid.cells[j as usize][i as usize].surrounding_mines =
                    grid.count_surrounding_mines(i, j);
            }
        }

        grid
    }

    //pub fn with_seed(width: u32, height: u32, mines: u8) -> MineGrid {
    //}

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn state(&self) -> GridState {
        self.state
    }

    pub fn mines(&self) -> u32 {
        self.mines
    }

    pub fn mines_left(&self) -> u32 {
        self.mines - self.mines_flagged
    }

    pub fn check_point(&self, x: u32, y: u32) -> bool {
        x < self.width && y < self.height
    }

    pub fn get_cell(&self, x: u32, y: u32) -> Option<Cell> {
        if self.check_point(x, y) {
            Some(self.cells[y as usize][x as usize].clone())
        } else {
            None
        }
    }

    pub fn get_neighbors(&self, x: u32, y: u32) -> Vec<Cell> {
        // TODO: Look into using a stack-allocated vector type?
        let mut neighbors = Vec::with_capacity(8);

        for j in -1..2i32 {
            for i in -1..2i32 {
                if i != 0 || j != 0 {
                    if let Some(cell) = self.get_cell((x as i32 + i) as u32,
                                                      (y as i32 + j) as u32) {
                        neighbors.push(cell);
                    }
                }
            }
        }
        neighbors
    }

    fn count_surrounding_mines(&self, x: u32, y: u32) -> u8 {
        self.get_neighbors(x, y).iter()
            .map(|cell| cell.mines)
            .sum()
    }

    fn count_surrounding_flags(&self, x: u32, y: u32) -> u8 {
        let mut flags = 0;
        for cell in self.get_neighbors(x, y) {
            if let CellState::Hidden(f) = cell.state {
                flags += f;
            }
        }
        flags
    }

    pub fn toggle_flag(&mut self, x: u32, y: u32) {
        if !self.check_point(x, y) {
            return;
        }

        let ref mut cell = self.cells[y as usize][x as usize];
        if let CellState::Hidden(flags) = cell.state {
            cell.state = CellState::Hidden((flags + 1) % (self.max_mines + 1))
        }
    }

    pub fn reveal(&mut self, x: u32, y: u32) {
        if !self.check_point(x, y) {
            return;
        }

        let cell = self.cells[y as usize][x as usize].clone();
        match cell.state {
            CellState::Hidden(0) => {
                // Try to reveal.
                self.cells[y as usize][x as usize].state = CellState::Revealed;

                if cell.mines != 0 {
                    self.state = GridState::Lose;
                    return;
                }

                self.spaces_left -= 1;
                if self.spaces_left == 0 {
                    self.state = GridState::Win;
                    return;
                }

                if cell.surrounding_mines == 0 {
                    for n in self.get_neighbors(x, y) {
                        self.reveal(n.x, n.y);
                    }
                }
            },
            CellState::Hidden(_) => {
                // Do nothing, since players can't reveal flagged cells.
            },
            CellState::Revealed => {
                if cell.surrounding_mines == 0 {
                    return;
                }
                let flags = self.count_surrounding_flags(x, y);
                if cell.surrounding_mines != flags {
                    return;
                }
                for neighbor in self.get_neighbors(x, y) {
                    if let CellState::Hidden(_) = neighbor.state {
                        self.reveal(neighbor.x, neighbor.y);
                    }
                }
            },
        }
    }
}

#[cfg(test)]
mod minegrid_test {
    use super::*;

    #[test]
    fn test_new() {
        let (width, height, mines) = (10, 10, 10);

        let grid = MineGrid::new(width, height, mines);

        let mut mine_count = 0;
        for j in 0..height {
            for i in 0..width {
                mine_count += grid.get_cell(i, j).unwrap().mines() as u32;
            }
        }
        assert_eq!(mines, mine_count);

        assert_eq!(width, grid.width());
        assert_eq!(height, grid.height());
        assert_eq!(mines, grid.mines());
        assert_eq!(GridState::Play, grid.state());
    }

    #[test]
    fn test_check_point() {
        let (width, height, mines) = (10, 10, 10);

        let grid = MineGrid::new(width, height, mines);

        assert_eq!(true, grid.check_point(0, 0));
        assert_eq!(true, grid.check_point(1, 0));
        assert_eq!(true, grid.check_point(9, 9));
        assert_eq!(false, grid.check_point(10, 0));
    }

    #[test]
    fn test_get_neighbors() {
        let (width, height, mines) = (10, 10, 10);

        let grid = MineGrid::new(width, height, mines);

        assert_eq!(3, grid.get_neighbors(0, 0).len());
        assert_eq!(5, grid.get_neighbors(1, 0).len());
        assert_eq!(8, grid.get_neighbors(1, 1).len());
    }

    #[test]
    fn test_toggle_flag() {
        let (width, height, mines) = (10, 10, 10);
        let mut grid = MineGrid::new(width, height, mines);

        assert_eq!(CellState::Hidden(0), grid.get_cell(0, 0).unwrap().state());
        grid.toggle_flag(0, 0);
        assert_eq!(CellState::Hidden(1), grid.get_cell(0, 0).unwrap().state());
        grid.toggle_flag(0, 0);
        assert_eq!(CellState::Hidden(0), grid.get_cell(0, 0).unwrap().state());
    }

    #[test]
    fn test_reveal_empty_grid() {
        let (width, height, mines) = (10, 10, 0);
        let mut grid = MineGrid::new(width, height, mines);

        assert_eq!(CellState::Hidden(0), grid.get_cell(0, 0).unwrap().state());
        assert_eq!(GridState::Play, grid.state());
        grid.reveal(0, 0);
        for j in 0..height {
            for i in 0..width {
                assert_eq!(CellState::Revealed, grid.get_cell(i, j).unwrap().state());
            }
        }
        assert_eq!(GridState::Win, grid.state());
    }

    #[test]
    fn test_reveal_mine() {
        let (width, height, mines) = (10, 10, 100);
        let mut grid = MineGrid::new(width, height, mines);

        assert_eq!(CellState::Hidden(0), grid.get_cell(0, 0).unwrap().state());
        assert_eq!(GridState::Play, grid.state());
        grid.reveal(0, 0);
        assert_eq!(CellState::Revealed, grid.get_cell(0, 0).unwrap().state());
        assert_eq!(GridState::Lose, grid.state());
    }
}
