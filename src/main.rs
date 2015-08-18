extern crate rand;
extern crate rustbox;

use std::default::Default;

use rustbox::{
    Color,
    Event,
    Key,
    RustBox,
};

use rustbox_cell::{Cell, print_cells};
use minegrid::{GridState, MineGrid};

mod minegrid;
mod rustbox_cell;

#[derive(Clone, Copy, Debug, PartialEq)]
enum GameState {
    Play,
    Lose,
    Win,
    New,
    Quit,
}

enum Difficulty {
    Easy,
    Medium,
    Hard,
    //Custom(u32, u32, u32),
}

static ACTION_STRINGS: &'static [&'static [&'static str]] = &[
    // GameState::Play
    &[
        "Space: reveal",
        "f: flag",
        "Arrow keys: move",
        "",
        "n: new game",
        "q: quit",
    ],
    // GameState::Lose
    &[
        "n: new game",
        "q: quit",
    ],
    // GameState::Win
    &[
        "n: new game",
        "q: quit",
    ],
    // GameState::New
    &[
        "e: easy",
        "m: medium",
        "h: hard",
        "",
        "c: cancel",
        "q: quit",
    ],
];

struct Game {
    rb: RustBox,
    grid: MineGrid,
    grid_pos: (usize, usize),
    actions_pos: (usize, usize),
    status_pos: (usize, usize),
    mines_pos: (usize, usize),
    cursor_pos: (usize, usize),
    //grid_changed: bool,
    state: GameState,
}

impl Game {
    fn new(rb: RustBox) -> Game {
        let mut game = Game {
            rb: rb,
            grid: MineGrid::new(0, 0, 0),
            grid_pos: (20, 1),
            actions_pos: (0, 2),
            status_pos: (0, 0),
            mines_pos: (0, 0),
            cursor_pos: (0, 0),
            //grid_changed: false,
            state: GameState::Play,
        };

        game.reset(Difficulty::Easy);

        game
    }

    fn reset(&mut self, difficulty: Difficulty) {
        match difficulty {
            Difficulty::Easy => self.grid = MineGrid::new(9, 9, 10),
            Difficulty::Medium => self.grid = MineGrid::new(16, 16, 40),
            Difficulty::Hard => self.grid = MineGrid::new(40, 16, 99),
            //Difficulty::Custom(width, height, mines) =>
            //    self.grid = MineGrid::new(width, height, mines),
        }

        self.status_pos = (0, self.grid_pos.1 + self.grid.height() as usize + 3);
        self.mines_pos = (self.grid_pos.0 + self.grid.width() as usize / 2, 0);
        self.cursor_pos = (0, 0);
        self.state = GameState::Play;
    }

    fn move_cursor_up(&mut self) {
        if self.cursor_pos.1 > 0 {
            self.cursor_pos.1 -= 1;
        }
    }

    fn move_cursor_down(&mut self) {
        if self.cursor_pos.1 < self.grid.height() as usize - 1 {
            self.cursor_pos.1 += 1;
        }
    }

    fn move_cursor_left(&mut self) {
        if self.cursor_pos.0 > 0 {
            self.cursor_pos.0 -= 1;
        }
    }

    fn move_cursor_right(&mut self) {
        if self.cursor_pos.0 < self.grid.width() as usize - 1 {
            self.cursor_pos.0 += 1;
        }
    }

    fn update(&mut self) {
        match self.state {
            GameState::Play => self.play_update(),
            GameState::Lose => self.lose_update(),
            GameState::Win => self.win_update(),
            GameState::New => self.new_update(),
            GameState::Quit => {},
        }
    }

    fn play_update(&mut self) {
        match self.rb.poll_event(false).unwrap() {
            Event::KeyEvent(Some(key)) => {
                match key {
                    Key::Char(' ') => {
                        self.grid.reveal(self.cursor_pos.0 as u32,
                                         self.cursor_pos.1 as u32);
                        match self.grid.state() {
                            GridState::Play => {},
                            GridState::Win => self.state = GameState::Win,
                            GridState::Lose => self.state = GameState::Lose,
                        }
                    },
                    Key::Char('f') => self.grid.toggle_flag(
                        self.cursor_pos.0 as u32, self.cursor_pos.1 as u32),
                    Key::Up => self.move_cursor_up(),
                    Key::Down => self.move_cursor_down(),
                    Key::Left => self.move_cursor_left(),
                    Key::Right => self.move_cursor_right(),
                    Key::Char('n') => self.state = GameState::New,
                    Key::Char('q') => self.state = GameState::Quit,
                    _ => return,
                }
            },
            _ => return,
        }
    }

    fn lose_update(&mut self) {
        match self.rb.poll_event(false).unwrap() {
            Event::KeyEvent(Some(key)) => {
                match key {
                    Key::Char('n') => self.state = GameState::New,
                    Key::Char('q') => self.state = GameState::Quit,
                    _ => return,
                }
            },
            _ => return,
        }
    }

    fn win_update(&mut self) {
        match self.rb.poll_event(false).unwrap() {
            Event::KeyEvent(Some(key)) => {
                match key {
                    Key::Char('n') => self.state = GameState::New,
                    Key::Char('q') => self.state = GameState::Quit,
                    _ => return,
                }
            },
            _ => return,
        }
    }

    fn new_update(&mut self) {
        match self.rb.poll_event(false).unwrap() {
            Event::KeyEvent(Some(key)) => {
                match key {
                    Key::Char('e') => self.reset(Difficulty::Easy),
                    Key::Char('m') => self.reset(Difficulty::Medium),
                    Key::Char('h') => self.reset(Difficulty::Hard),
                    Key::Char('c') => self.state = match self.grid.state() {
                        GridState::Play => GameState::Play,
                        GridState::Lose => GameState::Lose,
                        GridState::Win => GameState::Win,
                    },
                    Key::Char('q') => self.state = GameState::Quit,
                    _ => return,
                }
            },
            _ => return,
        }
    }

    fn display(&self) {
        self.rb.clear();

        // Title
        self.rb.print(0, 0, rustbox::RB_BOLD, Color::Default, Color::Default, "Minesweeper");

        self.draw_actions();

        // Mine counter
        self.rb.print(self.mines_pos.0, self.mines_pos.1,
                      rustbox::RB_BOLD, Color::Red, Color::White,
                      &format!("{:02}", self.grid.mines_left()));

        self.draw_grid();

        self.draw_status();

        if self.state == GameState::Play {
            self.rb.set_cursor((self.cursor_pos.0 + self.grid_pos.0 + 1) as isize,
                               (self.cursor_pos.1 + self.grid_pos.1 + 1) as isize);
        } else {
            self.rb.set_cursor(-1, -1);
        }

        self.rb.present();
    }

    fn draw_grid(&self) {
        let border_cell = Cell {
            ch: '#',
            style: rustbox::RB_NORMAL,
            fg: Color::Default,
            bg: Color::Default,
        };
        let flag_cell = Cell {
            ch: 'F',
            style: rustbox::RB_BOLD,
            fg: Color::Red,
            bg: Color::Blue,
        };
        let mine_cell = Cell {
            ch: '*',
            style: rustbox::RB_BOLD,
            fg: Color::Red,
            bg: Color::Default,
        };
        let hidden_cell = Cell {
            ch: ' ',
            style: rustbox::RB_NORMAL,
            fg: Color::Default,
            bg: Color::Blue,
        };
        let revealed_cell = Cell {
            ch: ' ',
            style: rustbox::RB_NORMAL,
            fg: Color::Default,
            bg: Color::Default,
        };

        let mut line_pos = 0;
        let mut line = Vec::with_capacity(self.grid.width() as usize + 2);

        line.push(border_cell);
        for _ in 0..self.grid.width() {
            line.push(border_cell);
        }
        line.push(border_cell);

        print_cells(&self.rb, self.grid_pos.0, self.grid_pos.1 + line_pos, &line);

        for j in 0..self.grid.height() {
            line_pos += 1;
            line.clear();

            line.push(border_cell);
            for i in 0..self.grid.width() {
                let cell = self.grid.get_cell(i, j).unwrap();
                line.push(if cell.flags() != 0 {
                    flag_cell
                } else if !cell.revealed() {
                    hidden_cell
                } else if cell.mines() != 0 {
                    mine_cell
                } else if cell.surrounding_mines() != 0 {
                    self.mine_cell_format(cell.surrounding_mines())
                } else {
                    revealed_cell
                });
            }
            line.push(border_cell);

            print_cells(&self.rb, self.grid_pos.0, self.grid_pos.1 + line_pos, &line);
        }

        line_pos += 1;
        line.clear();

        line.push(border_cell);
        for _ in 0..self.grid.width() {
            line.push(border_cell);
        }
        line.push(border_cell);
        print_cells(&self.rb, self.grid_pos.0, self.grid_pos.1 + line_pos, &line);
    }

    fn mine_cell_format(&self, mines: u8) -> Cell {
        let colors = match mines {
            1 => (Color::Blue, Color::Default),
            2 => (Color::Green, Color::Default),
            3 => (Color::Red, Color::Default),
            4 => (Color::Yellow, Color::Default),
            5 => (Color::Magenta, Color::Default),
            6 => (Color::Cyan, Color::Default),
            7 => (Color::White, Color::Cyan),
            8 => (Color::White, Color::Magenta),
            _ => (Color::Default, Color::Default),
        };
        Cell {
            ch: mines.to_string().chars().next().unwrap(),
            style: rustbox::RB_NORMAL,
            fg: colors.0,
            bg: colors.1,
        }
    }

    fn draw_actions(&self) {
        for (i, text) in ACTION_STRINGS[self.state as usize].iter().enumerate() {
            self.rb.print(self.actions_pos.0, self.actions_pos.1 + i,
                          rustbox::RB_NORMAL, Color::Default, Color::Default, text);
        }
    }

    fn draw_status(&self) {
        let status = match self.state {
            GameState::Play => "Play!",
            GameState::Lose => "You lose...",
            GameState::Win => "You win!",
            GameState::New => "Choose a difficulty",
            _ => "",
        };
        self.rb.print(self.status_pos.0, self.status_pos.1,
                      rustbox::RB_NORMAL, Color::Default, Color::Default, &status);
    }
}

fn main() {
    let rb = RustBox::init(Default::default()).unwrap();

    let mut game = Game::new(rb);

    while game.state != GameState::Quit {
        game.display();
        game.update();
    }
}
