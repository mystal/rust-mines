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
use std::char;

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

struct Game {
    grid: MineGrid,
    grid_pos: (uint, uint),
    actions_pos: (uint, uint),
    mines_pos: (uint, uint),
    cursor_pos: (uint, uint),
    grid_changed: bool,
    state: GameState,
    clear_screen: bool,
}

impl Game {
    fn new() -> Game {
        let mut game = Game {
            grid: MineGrid::new(0, 0, 0),
            grid_pos: (20, 1),
            actions_pos: (0, 2),
            mines_pos: (0, 0),
            cursor_pos: (0, 0),
            grid_changed: false,
            state: GameState::Play,
            clear_screen: false,
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

        self.mines_pos = (self.grid_pos.0 + self.grid.width() as uint / 2, 0);
        self.cursor_pos = (0, 0);
    }

    fn update(&mut self) {
        match tb::poll_event() {
            Event::KeyEvent(_, _, ch) => {
                match ch {
                    Some('q') => self.state = GameState::Quit,
                    _ => return,
                }
            },
            _ => return,
        }
        //let action = action_funcs[game.state][tb::poll_event()];
        //if let Some(act) {
        //    let next_state = act();
        //    clear = next_state != game.state;
        //    game.state = next_state;
        //}
    }

    fn display(&self/*, clear: bool*/) {
        tb::clear();
        //tb::print(1, 1, tb::Bold, tb::White, tb::Black, "Hello, World!".to_string());
        //tb::present();
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
