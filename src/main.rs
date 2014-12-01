#![feature(if_let, tuple_indexing)]

extern crate rustbox;

use minegrid::GridState;
use minegrid::MineGrid;
use rustbox::Event;
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
        match rustbox::poll_event() {
            Event::KeyEvent(_, _, ch) => {
                match char::from_u32(ch) {
                    Some('q') => self.state = GameState::Quit,
                    _ => return,
                }
            },
            _ => return,
        }
        //let action = action_funcs[game.state][rustbox::poll_event()];
        //if let Some(act) {
        //    let next_state = act();
        //    clear = next_state != game.state;
        //    game.state = next_state;
        //}
    }

    fn display(&self/*, clear: bool*/) {
        //rustbox::print(1, 1, rustbox::Bold, rustbox::White, rustbox::Black, "Hello, World!".to_string());
        //rustbox::present();
    }
}

fn main() {
    rustbox::init();

    let mut game = Game::new();

    //bool clear = true;
    while game.state != GameState::Quit {
        game.display(/*clear*/);
        game.update();
    }

    rustbox::shutdown();
}
