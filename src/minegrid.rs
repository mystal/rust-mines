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
    max_flags: u8,
    mines_flagged: u32,
    spaces_left: u32,
    state: GridState,
    //seed: u64,
}

impl MineGrid {
    pub fn new(width: u32, height: u32, mines: u32) -> MineGrid {
        let mut cells = Vec::with_capacity(height as uint);
        
        // TODO: randomly place mines

        let mut grid = MineGrid {
            cells: cells,
            width: width,
            height: height,
            mines: mines,
            max_flags: 1,
            mines_flagged: 0,
            spaces_left: width * height - mines,
            state: GridState::Play,
        };

        // TODO: count surrounding mines

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

    pub fn check_point(&self, x: u32, y: u32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    pub fn get_cell(&self, x: u32, y: u32) -> Option<&Cell> {
        if self.check_point(x, y) {
            Some(&self.cells[y as uint][x as uint])
        } else {
            None
        }
    }

    //pub fn get_neighbors(&self, x: u32, y: u32) -> Vec<&Cell> {
    //    let mut neighbors = Vec::with_capacity(8);
    //    if self.check_point(x, y) {
    //        for j in range(-1, 2) {
    //            for i in range(-1, 2) {
    //            }
    //        }
    //    }
    //    neighbors
    //}

    //pub fn toggle_flag(&mut self, x: u32, y: u32) {
    //    if self.check_point(x, y) {
    //        let &mut cell = self.cells[y][x];
    //        if !cell.revealed {
    //            cell.flags = (cell.flags + 1) % self.max_flags;
    //        }
    //    }
    //}

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

        assert_eq!(width, grid.width());
        assert_eq!(height, grid.height());
        assert_eq!(mines, grid.mines());
        assert_eq!(GridState::Play, grid.state());
    }
}
