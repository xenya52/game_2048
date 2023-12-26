use std::vec;
use rand::{thread_rng, Rng, seq::SliceRandom};

use crossterm::{
    event::{self, KeyCode, KeyEvent, KeyModifiers, read},
    ErrorKind,
};


use std::io::{self, Write};


type Tile = u32;
type Board = Vec<Vec<Tile>>;
enum UserAction {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Quit,
    None,
}

fn init_board() -> Board {
    let mut board = vec![vec![0; 4]; 4];
    // Add two random tiles
    add_random_tile(&mut board);
    add_random_tile(&mut board);
    board
}

fn is_game_over(board: &mut Board) -> bool {
    // Check for any empty spaces
    if board.iter().any(|row| row.iter().any(|&tile| tile == 0)) {
        return false;
    }

    // Check for possible merges horizontally and vertically
    for i in 0..4 {
        for j in 0..4 {
            // Check horizontally
            if j < 3 && board[i][j] == board[i][j + 1] {
                return false;
            }
            // Check vertically
            if i < 3 && board[i][j] == board[i + 1][j] {
                return false;
            }
        }
    }

    // No empty spaces and no possible merges
    true
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

fn get_user_input() -> Result<UserAction, ErrorKind> {
    if event::poll(std::time::Duration::from_millis(100))? {
        if let event::Event::Key(KeyEvent { code, modifiers, .. }) = read()? {
            match code {
                KeyCode::Up | KeyCode::Char('w') => Ok(UserAction::MoveUp),
                KeyCode::Down | KeyCode::Char('s') => Ok(UserAction::MoveDown),
                KeyCode::Left | KeyCode::Char('a') => Ok(UserAction::MoveLeft),
                KeyCode::Right | KeyCode::Char('d') => Ok(UserAction::MoveRight),
                KeyCode::Esc | KeyCode::Char('q') => Ok(UserAction::Quit),
                _ => Ok(UserAction::None),
            }
        } else {
            Ok(UserAction::None)
        }
    } else {
        Ok(UserAction::None)
    }
}

fn move_up(board: &mut Board) {
    println!("up");
}
fn move_down(board: &mut Board) {
    println!("down")
}
fn move_left(board: &mut Board) {
    for row in board.iter_mut() {
        // Step 1: Shift all tiles to the left
        let mut last_non_zero = 0;
        for i in 0..4 {
            if row[i] != 0 {
                row.swap(i, last_non_zero);
                last_non_zero += 1;
            }
        }

        // Step 2: Merge tiles
        for i in 0..3 {
            if row[i] == row[i + 1] && row[i] != 0 {
                row[i] *= 2;
                row[i + 1] = 0;
            }
        }

        // Step 3: Shift again after merging
        let mut last_non_zero = 0;
        for i in 0..4 {
            if row[i] != 0 {
                row.swap(i, last_non_zero);
                last_non_zero += 1;
            }
        }
    }
    println!("left");
}
fn move_right(board: &mut Board) {
    /*
    for row in board.iter_mut() {
        // Step 1: Shift all tiles to the left
        let mut last_non_zero = 4;
        for i in 4..0 {
            if row[i] != 0 {
                row.swap(i, last_non_zero);
                last_non_zero -= 1;
            }
        }

        // Step 2: Merge tiles
        for i in 3..0 {
            if row[i] == row[i - 1] && row[i] != 0 {
                row[i] *= 2;
                row[i - 1] = 0;
            }
        }

        // Step 3: Shift again after merging
        let mut last_non_zero = 4;
        for i in 4..0 {
            if row[i] != 0 {
                row.swap(i, last_non_zero);
                last_non_zero -= 1;
            }
        }
    }
    */
    println!("right");
}

fn game_loop(board: &mut Vec<Vec<u32>>) -> Result<(), crossterm::ErrorKind> {
    while !is_game_over(board) {
        match get_user_input()? {
            UserAction::MoveUp => move_up(board),
            UserAction::MoveDown => move_down(board),
            UserAction::MoveLeft => move_left(board),
            UserAction::MoveRight => move_right(board),
            UserAction::Quit => break,
            UserAction::None => continue,
        }
        add_random_tile(board);
        print_board(board);
    }
    Ok(())
}

fn main() {
    let mut board = init_board();
        print_board(&mut board);
        game_loop(&mut board);
}