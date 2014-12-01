use std::collections::HashSet;
use std::rand;
use std::rand::Rng;

pub struct Cell {
    x: u32,
    y: u32,
    mines: u8,
    flags: u8,
    revealed: bool,
    surrounding_mines: u8,
}

#[deriving(PartialEq, Show)]
pub enum GridState {
    Play,
    Win,
    Lose,
}

pub struct MineGrid {
    cells: Vec<Vec<Cell>>,
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

    pub fn flags(&self) -> u8 {
        self.flags
    }

    pub fn revealed(&self) -> bool {
        self.revealed
    }
}

impl MineGrid {
    pub fn new(width: u32, height: u32, mines: u32) -> MineGrid {
        let mut cells = Vec::with_capacity(height as uint);

        // Randomly place mines
        let mut rng = rand::task_rng();
        let mut mine_points = HashSet::new();
        while mine_points.len() != mines as uint {
            let point = (rng.gen_range(0, width),
                         rng.gen_range(0, height));
            mine_points.insert(point);
        }

        for j in range(0, height) {
            let mut row = Vec::with_capacity(width as uint);
            for i in range(0, width) {
                row.push(Cell {
                    x: i,
                    y: j,
                    mines: if mine_points.contains(&(i, j)) { 1 } else { 0 },
                    flags: 0,
                    revealed: false,
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

        // Count surrounding mines
        for j in range(0, height) {
            for i in range(0, width) {
                grid.cells[j as uint][i as uint].surrounding_mines =
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

    pub fn get_cell(&self, x: u32, y: u32) -> Option<&Cell> {
        if self.check_point(x, y) {
            Some(&self.cells[y as uint][x as uint])
        } else {
            None
        }
    }

    pub fn get_neighbors(&self, x: u32, y: u32) -> Vec<&Cell> {
        let mut neighbors = Vec::with_capacity(8);
        for j in range(-1, 2i32) {
            for i in range(-1, 2i32) {
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
        let mut mines = 0;
        for n in self.get_neighbors(x, y).iter() {
            mines += n.mines;
        }
        mines
    }

    fn count_surrounding_flags(&self, x: u32, y: u32) -> u8 {
        let mut flags = 0;
        for n in self.get_neighbors(x, y).iter() {
            flags += n.flags;
        }
        flags
    }

    pub fn toggle_flag(&mut self, x: u32, y: u32) {
        if !self.check_point(x, y) {
            return;
        }

        let max_mines = self.max_mines;
        let cell = &mut self.cells[y as uint][x as uint];
        if !cell.revealed {
            cell.flags = (cell.flags + 1) % (max_mines + 1);
        }
    }

    //pub fn reveal(&mut self, x: u32, y: u32) {
    //}
}

#[cfg(test)]
mod minegrid_test {
    use super::GridState;
    use super::MineGrid;

    #[test]
    fn test_new() {
        let (width, height, mines) = (10, 10, 10);

        let grid = MineGrid::new(width, height, mines);

        let mut mine_count = 0;
        for j in range(0, height) {
            for i in range(0, width) {
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
        assert_eq!(false, grid.check_point(-1, 0));
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

        assert_eq!(0, grid.get_cell(0, 0).unwrap().flags());
        grid.toggle_flag(0, 0);
        assert_eq!(1, grid.get_cell(0, 0).unwrap().flags());
        grid.toggle_flag(0, 0);
        assert_eq!(0, grid.get_cell(0, 0).unwrap().flags());
    }
}
