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

fn handle_input(board: &mut Vec<Vec<u32>>) -> Result<(), crossterm::ErrorKind> {
    loop {
        match get_user_input()? {
            UserAction::MoveUp => move_up(board),
            UserAction::MoveDown => move_down(board),
            UserAction::MoveLeft => move_left(board),
            UserAction::MoveRight => move_right(board),
            UserAction::Quit => break,
            UserAction::None => continue,
        }
    }
    Ok(())
}

fn main() {
    let mut board = init_board();

    while is_game_over(&mut board) {
        print_board(&mut board);
        get_user_input();
        handle_input(&mut board);
        add_random_tile(&mut board);
    }
}