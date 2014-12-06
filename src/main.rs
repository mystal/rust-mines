#![feature(if_let, tuple_indexing)]

extern crate termbox;

use minegrid::GridState;
use minegrid::MineGrid;
use termbox as tb;
use termbox::{
    Color,
    Event,
    Key,
    Style,
};

mod minegrid;

#[deriving(PartialEq, Show)]
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
    grid_changed: bool,
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
            grid_changed: false,
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
        }

        self.status_pos = (0, self.grid_pos.1 + self.grid.height() as uint + 3);
        self.mines_pos = (self.grid_pos.0 + self.grid.width() as uint / 2, 0);
        self.cursor_pos = (0, 0);
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
            //Win => self.win_update(),
            //New => self.new_update(),
            //Quit => {},
            _ => {
                println!("State {} not implemented!", self.state);
                self.state = GameState::Quit
            },
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

    fn display(&self) {
        tb::clear();

        // Title
        tb::print(0, 0, Style::Bold, Color::Default, Color::Default,
                  "Minesweeper");

        self.draw_actions();

        // Mine counter
        tb::print(self.mines_pos.0, self.mines_pos.1,
                  Style::Bold, Color::Red, Color::White,
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
        let mut line_pos = 0;
        let mut line = String::with_capacity(self.grid.width() as uint + 2);

        line.push('#');
        for i in range(0, self.grid.width()) {
            line.push('#');
        }
        line.push('#');
        tb::print(self.grid_pos.0, self.grid_pos.1 + line_pos,
                  Style::Normal, Color::Default, Color::Default, line.as_slice());

        for j in range(0, self.grid.height()) {
            line_pos += 1;
            line.clear();

            line.push('#');
            for i in range(0, self.grid.width()) {
                let cell = self.grid.get_cell(i, j).unwrap();
                let surrounding_mines_string = cell.surrounding_mines().to_string();
                line.push_str(if cell.flags() != 0 {
                    "F"
                } else if !cell.revealed() {
                    "-"
                } else if cell.mines() != 0 {
                    "*"
                } else if cell.surrounding_mines() != 0 {
                    surrounding_mines_string.as_slice()
                } else {
                    " "
                });
            }

            line.push('#');
            tb::print(self.grid_pos.0, self.grid_pos.1 + line_pos,
                      Style::Normal, Color::Default, Color::Default, line.as_slice());
        }

        line_pos += 1;
        line.clear();

        line.push('#');
        for i in range(0, self.grid.width()) {
            line.push('#');
        }
        line.push('#');
        tb::print(self.grid_pos.0, self.grid_pos.1 + line_pos,
                  Style::Normal, Color::Default, Color::Default, line.as_slice());
    }

    fn draw_actions(&self) {
        for (i, text) in ACTION_STRINGS[self.state as uint].iter().enumerate() {
            tb::print(self.actions_pos.0, self.actions_pos.1 + i,
                      Style::Normal, Color::Default, Color::Default, *text);
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
        tb::print(self.status_pos.0, self.status_pos.1,
                  Style::Normal, Color::Default, Color::Default,
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
