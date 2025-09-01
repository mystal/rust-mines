use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use minegrid::{CellState, GridState, MineGrid};
use ratatui::{
    DefaultTerminal,
    Frame,
    buffer::{Buffer, Cell},
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Modifier, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, Widget},
};

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
    Custom {
        width: u32,
        height: u32,
        mines: u32,
    },
}

static ACTION_STRINGS: &[&[&str]] = &[
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

fn format_mine_cell(mines: u8) -> Cell {
    let (num_str, fg, bg) = match mines {
        1 => ("1", Color::Blue, Color::Reset),
        2 => ("2", Color::Green, Color::Reset),
        3 => ("3", Color::Red, Color::Reset),
        4 => ("4", Color::Yellow, Color::Reset),
        5 => ("5", Color::Magenta, Color::Reset),
        6 => ("6", Color::Cyan, Color::Reset),
        7 => ("7", Color::White, Color::Cyan),
        8 => ("8", Color::White, Color::Magenta),
        _ => panic!("Unexpected number of surrounding mines: {}!", mines),
    };

    let mut cell = Cell::new(num_str);
    cell.fg = fg;
    cell.bg = bg;
    cell
}

struct Game {
    grid: MineGrid,
    grid_pos: (u16, u16),
    actions_pos: (usize, usize),
    status_pos: (usize, usize),
    mines_pos: (usize, usize),
    cursor_pos: (u16, u16),
    //grid_changed: bool,
    state: GameState,
}

impl Game {
    fn new() -> Game {
        let mut game = Game {
            state: GameState::Play,
            grid: MineGrid::new(0, 0, 0),
            grid_pos: (0, 0),
            actions_pos: (0, 2),
            status_pos: (0, 0),
            mines_pos: (0, 0),
            cursor_pos: (0, 0),
            //grid_changed: false,
        };

        game.reset(Difficulty::Easy);

        game
    }

    fn run(&mut self, mut terminal: DefaultTerminal) {
        terminal.clear().unwrap();
        while self.state != GameState::Quit {
            terminal.draw(|frame| {
                self.draw(frame);
            }).unwrap();
            self.handle_events();
        }
    }

    fn handle_events(&mut self) {
        match event::read().unwrap() {
            // It's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.state = GameState::Quit,
            KeyCode::Char(' ') => self.grid.reveal(self.cursor_pos.0 as u32, self.cursor_pos.1 as u32),
            KeyCode::Char('f') => self.grid.toggle_flag(self.cursor_pos.0 as u32, self.cursor_pos.1 as u32),
            KeyCode::Left => self.move_cursor_left(),
            KeyCode::Right => self.move_cursor_right(),
            KeyCode::Up => self.move_cursor_up(),
            KeyCode::Down => self.move_cursor_down(),
            _ => {}
        }
    }

    fn reset(&mut self, difficulty: Difficulty) {
        match difficulty {
            Difficulty::Easy => self.grid = MineGrid::new(9, 9, 10),
            Difficulty::Medium => self.grid = MineGrid::new(16, 16, 40),
            Difficulty::Hard => self.grid = MineGrid::new(40, 16, 99),
            Difficulty::Custom { width, height, mines } => self.grid = MineGrid::new(width, height, mines),
        }

        // self.status_pos = (0, self.grid_pos.1 + self.grid.height() as usize + 3);
        // self.mines_pos = (self.grid_pos.0 + self.grid.width() as usize / 2, 0);
        self.cursor_pos = (0, 0);
        self.state = GameState::Play;
    }

    fn move_cursor_up(&mut self) {
        if self.cursor_pos.1 > 0 {
            self.cursor_pos.1 -= 1;
        }
    }

    fn move_cursor_down(&mut self) {
        if (self.cursor_pos.1 as u32) < self.grid.height() - 1 {
            self.cursor_pos.1 += 1;
        }
    }

    fn move_cursor_left(&mut self) {
        if self.cursor_pos.0 > 0 {
            self.cursor_pos.0 -= 1;
        }
    }

    fn move_cursor_right(&mut self) {
        if (self.cursor_pos.0 as u32) < self.grid.width() - 1 {
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
        // match self.rb.poll_event(false).unwrap() {
        //     Event::KeyEvent(key) => {
        //         match key {
        //             Key::Char(' ') => {
        //                 self.grid.reveal(self.cursor_pos.0 as u32,
        //                                  self.cursor_pos.1 as u32);
        //                 match self.grid.state() {
        //                     GridState::Play => {},
        //                     GridState::Win => self.state = GameState::Win,
        //                     GridState::Lose => self.state = GameState::Lose,
        //                 }
        //             },
        //             Key::Char('f') => self.grid.toggle_flag(
        //                 self.cursor_pos.0 as u32, self.cursor_pos.1 as u32),
        //             Key::Up => self.move_cursor_up(),
        //             Key::Down => self.move_cursor_down(),
        //             Key::Left => self.move_cursor_left(),
        //             Key::Right => self.move_cursor_right(),
        //             Key::Char('n') => self.state = GameState::New,
        //             Key::Char('q') => self.state = GameState::Quit,
        //             _ => return,
        //         }
        //     },
        //     _ => return,
        // }
    }

    fn lose_update(&mut self) {
        // match self.rb.poll_event(false).unwrap() {
        //     Event::KeyEvent(key) => {
        //         match key {
        //             Key::Char('n') => self.state = GameState::New,
        //             Key::Char('q') => self.state = GameState::Quit,
        //             _ => return,
        //         }
        //     },
        //     _ => return,
        // }
    }

    fn win_update(&mut self) {
        // match self.rb.poll_event(false).unwrap() {
        //     Event::KeyEvent(key) => {
        //         match key {
        //             Key::Char('n') => self.state = GameState::New,
        //             Key::Char('q') => self.state = GameState::Quit,
        //             _ => return,
        //         }
        //     },
        //     _ => return,
        // }
    }

    fn new_update(&mut self) {
        // match self.rb.poll_event(false).unwrap() {
        //     Event::KeyEvent(key) => {
        //         match key {
        //             Key::Char('e') => self.reset(Difficulty::Easy),
        //             Key::Char('m') => self.reset(Difficulty::Medium),
        //             Key::Char('h') => self.reset(Difficulty::Hard),
        //             Key::Char('c') => self.state = match self.grid.state() {
        //                 GridState::Play => GameState::Play,
        //                 GridState::Lose => GameState::Lose,
        //                 GridState::Win => GameState::Win,
        //             },
        //             Key::Char('q') => self.state = GameState::Quit,
        //             _ => return,
        //         }
        //     },
        //     _ => return,
        // }
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn draw_grid(&self) {
        // // Draw the top border.
        // print_cell_repeated_x(&self.rb, self.grid_pos.0, self.grid_pos.1, BORDER_CELL, self.grid.width() as usize + 2);

        // // Draw the bottom border.
        // print_cell_repeated_x(&self.rb, self.grid_pos.0, self.grid_pos.1 + self.grid.height() as usize + 1, BORDER_CELL, self.grid.width() as usize + 2);

        // // Draw the left border.
        // print_cell_repeated_y(&self.rb, self.grid_pos.0, self.grid_pos.1 + 1, BORDER_CELL, self.grid.height() as usize);

        // // Draw the right border.
        // print_cell_repeated_y(&self.rb, self.grid_pos.0 + self.grid.width() as usize + 1, self.grid_pos.1 + 1, BORDER_CELL, self.grid.height() as usize);

        // // Draw the grid using a CellRenderer.
        // for (x, y, cell) in CellRenderer::new(&self.grid) {
        //     let (x, y) = (x as usize, y as usize);
        //     self.rb.print_char(self.grid_pos.0 + x + 1, self.grid_pos.1 + y + 1, cell.style, cell.fg, cell.bg, cell.ch);
        // }
    }

    fn draw_actions(&self) {
        // for (i, text) in ACTION_STRINGS[self.state as usize].iter().enumerate() {
        //     self.rb.print(self.actions_pos.0, self.actions_pos.1 + i,
        //                   rustbox::RB_NORMAL, Color::Default, Color::Default, text);
        // }
    }

    fn draw_status(&self) {
        let status = match self.state {
            GameState::Play => "Play!",
            GameState::Lose => "You lose...",
            GameState::Win => "You win!",
            GameState::New => "Choose a difficulty",
            _ => "",
        };
        // self.rb.print(self.status_pos.0, self.status_pos.1,
        //               rustbox::RB_NORMAL, Color::Default, Color::Default, &status);
    }
}

impl Widget for &Game {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Title
        let title = Line::from(" Minesweeper ".bold());
        let instructions = Line::from(vec![
            // " Decrement ".into(),
            // "<Left>".blue().bold(),
            // " Increment ".into(),
            // "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);
        block.render(area, buf);

        // self.draw_actions();

        // Mine counter
        // self.rb.print(self.mines_pos.0, self.mines_pos.1,
        //               rustbox::RB_BOLD, Color::Red, Color::White,
        //               &format!("{:02}", self.grid.mines_left()));

        // self.draw_grid();

        // self.draw_status();

        // Draw the minegrid.
        let mut hidden_cell = Cell::new(" ");
        hidden_cell.bg = Color::Blue;
        let revealed_cell = Cell::new(" ");
        let mut mine_cell = Cell::new("💣");
        mine_cell.bg = Color::Red;
        let mut flag_cell = Cell::new("🚩");
        flag_cell.bg = Color::Blue;

        // Each minegrid cell takes two terminal cells.
        let grid_area = center(area, Constraint::Length((self.grid.width() * 2) as u16), Constraint::Length(self.grid.height() as u16));
        for j in 0..self.grid.height() {
            for i in 0..self.grid.width() {
                let grid_cell = &self.grid[(i, j)];
                let mut cell = match grid_cell.state() {
                    CellState::Hidden { flags: 0 } => hidden_cell.clone(),
                    CellState::Hidden { .. } => flag_cell.clone(),
                    CellState::Revealed => if grid_cell.mines() != 0 {
                        mine_cell.clone()
                    } else if grid_cell.surrounding_mines() != 0 {
                        format_mine_cell(grid_cell.surrounding_mines())
                    } else {
                        revealed_cell.clone()
                    },
                };
                // Use a white background to show the cursor position.
                if self.grid.state() == GridState::Play && self.cursor_pos == (i as u16, j as u16) {
                    cell.bg = Color::White;
                    cell.modifier = Modifier::BOLD;
                }
                let tui_cell_coords_left = (grid_area.left() + (i * 2) as u16, grid_area.top() + j as u16);
                if let Some(buf_cell_left) = buf.cell_mut(tui_cell_coords_left) {
                    *buf_cell_left = cell.clone();
                    if !(buf_cell_left.symbol() == flag_cell.symbol() || buf_cell_left.symbol() == mine_cell.symbol()) {
                        buf_cell_left.set_symbol(" ");
                    }
                }
                let tui_cell_coords_right = (tui_cell_coords_left.0 + 1, tui_cell_coords_left.1);
                if let Some(buf_cell_right) = buf.cell_mut(tui_cell_coords_right) {
                    *buf_cell_right = cell;
                    if buf_cell_right.symbol() == flag_cell.symbol() || buf_cell_right.symbol() == mine_cell.symbol() {
                        buf_cell_right.set_symbol(" ");
                    }
                }
            }
        }
    }
}

// Return a centered area following the given constraints.
//
// Based on:
fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}

fn main() {
    let terminal = ratatui::init();
    Game::new().run(terminal);
    ratatui::restore();
}
