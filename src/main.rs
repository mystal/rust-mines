extern crate termbox;

use minegrid::GridState;
use minegrid::MineGrid;
use termbox as tb;
use termbox::{
    Attribute,
    Cell,
    Color,
    Event,
    Key,
    Style,
};

mod minegrid;

#[derive(PartialEq, Show, Copy)]
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
    grid: MineGrid,
    grid_pos: (uint, uint),
    actions_pos: (uint, uint),
    status_pos: (uint, uint),
    mines_pos: (uint, uint),
    cursor_pos: (uint, uint),
    //grid_changed: bool,
    state: GameState,
}

impl Game {
    fn new() -> Game {
        let mut game = Game {
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

        self.status_pos = (0, self.grid_pos.1 + self.grid.height() as uint + 3);
        self.mines_pos = (self.grid_pos.0 + self.grid.width() as uint / 2, 0);
        self.cursor_pos = (0, 0);
        self.state = GameState::Play;
    }

    fn move_cursor_up(&mut self) {
        if self.cursor_pos.1 > 0 {
            self.cursor_pos.1 -= 1;
        }
    }

    fn move_cursor_down(&mut self) {
        if self.cursor_pos.1 < self.grid.height() as uint - 1 {
            self.cursor_pos.1 += 1;
        }
    }

    fn move_cursor_left(&mut self) {
        if self.cursor_pos.0 > 0 {
            self.cursor_pos.0 -= 1;
        }
    }

    fn move_cursor_right(&mut self) {
        if self.cursor_pos.0 < self.grid.width() as uint  - 1 {
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
        match tb::poll_event() {
            Event::KeyEvent(_, key, ch) => {
                match (key, ch) {
                    (Some(Key::Space), _) => {
                        self.grid.reveal(self.cursor_pos.0 as u32,
                                         self.cursor_pos.1 as u32);
                        match self.grid.state() {
                            GridState::Play => {},
                            GridState::Win => self.state = GameState::Win,
                            GridState::Lose => self.state = GameState::Lose,
                        }
                    },
                    (_, Some('f')) => self.grid.toggle_flag(
                        self.cursor_pos.0 as u32, self.cursor_pos.1 as u32),
                    (Some(Key::ArrowUp), _) => self.move_cursor_up(),
                    (Some(Key::ArrowDown), _) => self.move_cursor_down(),
                    (Some(Key::ArrowLeft), _) => self.move_cursor_left(),
                    (Some(Key::ArrowRight), _) => self.move_cursor_right(),
                    (_, Some('n')) => self.state = GameState::New,
                    (_, Some('q')) => self.state = GameState::Quit,
                    _ => return,
                }
            },
            _ => return,
        }
    }

    fn lose_update(&mut self) {
        match tb::poll_event() {
            Event::KeyEvent(_, key, ch) => {
                match (key, ch) {
                    (_, Some('n')) => self.state = GameState::New,
                    (_, Some('q')) => self.state = GameState::Quit,
                    _ => return,
                }
            },
            _ => return,
        }
    }

    fn win_update(&mut self) {
        match tb::poll_event() {
            Event::KeyEvent(_, key, ch) => {
                match (key, ch) {
                    (_, Some('n')) => self.state = GameState::New,
                    (_, Some('q')) => self.state = GameState::Quit,
                    _ => return,
                }
            },
            _ => return,
        }
    }

    fn new_update(&mut self) {
        match tb::poll_event() {
            Event::KeyEvent(_, key, ch) => {
                match (key, ch) {
                    (_, Some('e')) => self.reset(Difficulty::Easy),
                    (_, Some('m')) => self.reset(Difficulty::Medium),
                    (_, Some('h')) => self.reset(Difficulty::Hard),
                    (_, Some('c')) => self.state = match self.grid.state() {
                        GridState::Play => GameState::Play,
                        GridState::Lose => GameState::Lose,
                        GridState::Win => GameState::Win,
                    },
                    (_, Some('q')) => self.state = GameState::Quit,
                    _ => return,
                }
            },
            _ => return,
        }
    }

    fn display(&self) {
        tb::clear();

        // Title
        let fg = Attribute {
            color: Color::Default,
            style: Style::Bold,
        };
        let bg = Attribute {
            color: Color::Default,
            style: Style::Normal,
        };
        tb::print_string_styled(0, 0, fg, bg, "Minesweeper");

        self.draw_actions();

        // Mine counter
        let fg = Attribute {
            color: Color::Red,
            style: Style::Bold,
        };
        let bg = Attribute {
            color: Color::White,
            style: Style::Normal,
        };
        tb::print_string_styled(
            self.mines_pos.0, self.mines_pos.1, fg, bg,
            format!("{:02}", self.grid.mines_left()).as_slice());

        self.draw_grid();

        self.draw_status();

        if self.state == GameState::Play {
            tb::set_cursor(self.cursor_pos.0 + self.grid_pos.0 + 1,
                           self.cursor_pos.1 + self.grid_pos.1 + 1);
        } else {
            tb::set_cursor(-1, -1);
        }

        tb::present();
    }

    fn draw_grid(&self) {
        let border_cell = Cell {
            ch: '#',
            fg: Attribute {
                color: Color::Default,
                style: Style::Normal,
            },
            bg: Attribute {
                color: Color::Default,
                style: Style::Normal,
            },
        };
        let flag_cell = Cell {
            ch: 'F',
            fg: Attribute {
                color: Color::Red,
                style: Style::Bold,
            },
            bg: Attribute {
                color: Color::Blue,
                style: Style::Normal,
            },
        };
        let mine_cell = Cell {
            ch: '*',
            fg: Attribute {
                color: Color::Red,
                style: Style::Bold,
            },
            bg: Attribute {
                color: Color::Default,
                style: Style::Normal,
            },
        };
        let hidden_cell = Cell {
            ch: ' ',
            fg: Attribute {
                color: Color::Default,
                style: Style::Normal,
            },
            bg: Attribute {
                color: Color::Blue,
                style: Style::Normal,
            },
        };
        let revealed_cell = Cell {
            ch: ' ',
            fg: Attribute {
                color: Color::Default,
                style: Style::Normal,
            },
            bg: Attribute {
                color: Color::Default,
                style: Style::Normal,
            },
        };

        let mut line_pos = 0;
        let mut line = Vec::with_capacity(self.grid.width() as uint + 2);

        line.push(border_cell);
        for _ in range(0, self.grid.width()) {
            line.push(border_cell);
        }
        line.push(border_cell);
        tb::print_cells(self.grid_pos.0, self.grid_pos.1 + line_pos,
                        line.as_slice());

        for j in range(0, self.grid.height()) {
            line_pos += 1;
            line.clear();

            line.push(border_cell);
            for i in range(0, self.grid.width()) {
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

            tb::print_cells(self.grid_pos.0, self.grid_pos.1 + line_pos,
                            line.as_slice());
        }

        line_pos += 1;
        line.clear();

        line.push(border_cell);
        for _ in range(0, self.grid.width()) {
            line.push(border_cell);
        }
        line.push(border_cell);
        tb::print_cells(self.grid_pos.0, self.grid_pos.1 + line_pos,
                        line.as_slice());
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
            ch: mines.to_string().as_slice().char_at(0),
            fg: Attribute {
                color: colors.0,
                style: Style::Normal,
            },
            bg: Attribute {
                color: colors.1,
                style: Style::Normal,
            }
        }
    }

    fn draw_actions(&self) {
        for (i, text) in ACTION_STRINGS[self.state as uint].iter().enumerate() {
            tb::print_string(self.actions_pos.0, self.actions_pos.1 + i, *text);
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
        tb::print_string(self.status_pos.0, self.status_pos.1,
                         status.as_slice());
    }
}

fn main() {
    tb::init();

    let mut game = Game::new();

    while game.state != GameState::Quit {
        game.display();
        game.update();
    }

    tb::shutdown();
}
