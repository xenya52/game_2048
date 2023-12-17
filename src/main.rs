use std::vec;
use rand::{thread_rng, Rng, seq::SliceRandom};

use crossterm::{
    event::{self, KeyCode, KeyEvent, KeyModifiers},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::io::{self, Write};

type Tile = u32;
type Board = Vec<Vec<Tile>>;

fn init_board() -> Board {
    let mut board = vec![vec![0; 4]; 4];
    // Add two random tiles
    add_random_tile(&mut board);
    add_random_tile(&mut board);
    board
}

fn add_random_tile(board: &mut Board) {
    let mut empty_tiles = Vec::new();
    for (i, row) in board.iter().enumerate() {
        for (j, &tile) in row.iter().enumerate() {
            if tile == 0 {
                empty_tiles.push((i, j));
            }
        }
    }

    if let Some(&(x, y)) = empty_tiles.choose(&mut thread_rng()) {
        let value = if thread_rng().gen_bool(0.9) { 2 } else { 4 };
        board[x][y] = value;
    }
}

fn print_board(board: &mut Board) {
    println!("+----+----+----+----+");
    for (i,row) in board.iter().enumerate() {
        for (j, row) in row.iter().enumerate() {
            print!("|{:4}",row);
        }
        println!("|");
        println!("+----+----+----+----+");
    }

}

fn handle_input(board: &mut Vec<Vec<u32>>) -> Result<(), crossterm::ErrorKind> {
    // Entering raw mode
    io::stdout().execute(EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    loop {
        if event::poll(std::time::Duration::from_millis(100))? {
            if let event::Event::Key(KeyEvent { code, modifiers, .. }) = event::read()? {
                match code {
                    KeyCode::Char('w') | KeyCode::Up => move_up(board),
                    KeyCode::Char('s') | KeyCode::Down => move_down(board),
                    KeyCode::Char('a') | KeyCode::Left => move_left(board),
                    KeyCode::Char('d') | KeyCode::Right => move_right(board),
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    _ => {}
                }

                if modifiers.contains(KeyModifiers::CONTROL) && code == KeyCode::Char('c') {
                    break;
                }
            }
        }
    }

    // Exiting raw mode
    terminal::disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn main() {
    let mut board = init_board();
    print_board(&mut board);
}