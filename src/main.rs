use std::{vec, f32::MAX_10_EXP};
use rand::{thread_rng, Rng, seq::SliceRandom};
use colorized::*;
use std::io::{self, Write};
use crossterm::execute;
use crossterm::{
    terminal::{self, SetSize},
    ExecutableCommand,
    event::{self, KeyCode, KeyEvent, KeyModifiers, read},
    ErrorKind,
};


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

fn winner_screen() {
    print!("{}",Colors::YellowFg.value());
    println!("Art by Susie Oviatt");
    println!("                   .,,. ");
    println!("            .,v%;mmmmmmmm;%%vv,. ");
    println!("         ,vvv%;mmmvv;vvvmmm;%vvvv,    .,,.");
    println!("  ,, ,vvvnnv%;mmmvv;%%;vvmmm;%vvvv%;mmmmmmm, ");
    println!(",mmmmmm;%%vv%;mmmvv;%%;vvmmm;%v%;mmmmmmmmmmm ");
    println!("mmmmmmmmmmm;%%;mmmvv%;vvmmm;%mmmmmmmmmmmmmm' ");
    println!("`mmmmmmmmmmmmmm%;mmv;vmmm;mmmmmmm;%vvvvvv' ");
    println!("    `%%%%%;mmmmmmmm;v%v;mmmmmm;%vvvnnvv' ");
    println!("     vvvvvv%%%%;mmmm%;mmmmmm;%vvvnnnnvv ");
    println!("     `vvnnnnvvv%%%;m;mmmmm;%vvnnmmnnvv' ");
    println!("      vvnmmnnnnvvv%%mmmm;%vvnnmmmnnnvv ");
    println!("      `vvnmmmnnvvv%mmm;%vvnnmmmmnnnvv'");
    println!("       `vvnmmmmvv%mmm;%vvnnmmmmnnnvv' ");
    println!("        `vvnmmmvv%mm;%vvvnnmmmnnvvv' ");
    println!("          `vvnmmvv%m;%vvvvnmnvvvv' ");
    println!("           .;;vvvvvm;%vvvvvvvv' ");
    println!("        .;;;;;;;;;;;;;;;;;;;;, ");
    println!("       ;;;;;;';;;;;;;;;;;'`;;;;;, ");
    println!("      .;;;'    `;;;;;;;;'   `;;;;;. ");
    println!("     .;;'        `;;;;;'      `;;;; ");
    println!("     ;'           :`;;'         ;;' ");
    println!("     ;            : ;'    ,    ,'             .");
    println!("      `           :'.:   .;;,.        .,;;;;;;' ");
    println!("                  ::::   ;;,;;;,     ;;;,;;;;' ");
    println!("                  ;;;;   `;;;,;;    .,';;;;' ");
    println!("                  ;;;;      `';; ,;;' ");
    println!("                ,;;;;;         .;',. ");
    println!("                  `;;;;       .;'  ';,. ");
    println!("                   `;;;.     .;'   ,;;,;;,. ");
    println!("                    ;;;;    .;'    `;;;;,;;; ");
    println!("   You won!!!       ;;;;   .;'       `;;,;;'");
    println!("                    `;;;,;;'           `;' ");
    println!("                     ;;;; ");
    println!("                     ;;;;. ");
    println!("                     `;;;;;,. ");
    println!("                      ;;;;' ");
    println!("                      ;;;; ");
    println!("                      ;;;; ");
    print!("{}",Colors::Reset.value());
}

fn is_game_over(board: &mut Board) -> bool {
    if board.iter().any(|row| row.iter().any(|&tile| tile == 2048)) {
        winner_screen();
        return true;
    }
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

fn board_changed(new_board: &mut Board, old_board: &Board) -> bool {
    if new_board != old_board {
        return true;
    }
    else {
        return false;
    }
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
            match row {
                0    => print!("|{}",Colors::WhiteFg.value()),
                2    => print!("|{}",Colors::CyanFg.value()),
                4    => print!("|{}",Colors::BlueFg.value()),
                8    => print!("|{}",Colors::MagentaFg.value()),
                16   => print!("|{}",Colors::RedFg.value()),
                32   => print!("|{}",Colors::CyanFg.value()),
                64   => print!("|{}",Colors::BlueFg.value()),
                128  => print!("|{}",Colors::MagentaFg.value()),
                256  => print!("|{}",Colors::RedFg.value()),
                512  => print!("|{}",Colors::CyanFg.value()),
                1024 => print!("|{}",Colors::BlueFg.value()),
                2048 => print!("|{}",Colors::YellowFg.value()),
                _ => print!("{}",Colors::Reset.value())
            }
            print!("{:4}", row);
            print!("{}",Colors::Reset.value());
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
    for col in 0..4 {
        // Compact non-zero tiles upwards
        let mut compacted = board.iter().map(|row| row[col]).filter(|&tile| tile != 0).collect::<Vec<_>>();
        
        // Merge tiles and update compacted vector
        let mut merged = Vec::new();
        while let Some(first) = compacted.pop() {
            if compacted.last() == Some(&first) {
                merged.push(first * 2);
                compacted.pop();
            } else {
                merged.push(first);
            }
        }
        merged.reverse();

        // Fill in the gaps with zeros
        while merged.len() < 4 {
            merged.push(0);
        }

        // Update the board
        for (row, &tile) in merged.iter().enumerate() {
            board[row][col] = tile;
        }
    }
}
fn move_down(board: &mut Board) {
    for col in 0..4 {
        let mut temp_column: Vec<u32> = vec![0; 4]; // Temporary column to store new values
        let mut write_index = 3; // Start from the bottom of the column

        // Step 1: Shift down non-zero tiles and merge
        for row in (0..4).rev() {
            if board[row][col] != 0 {
                if temp_column[write_index] == 0 {
                    temp_column[write_index] = board[row][col];
                } else if temp_column[write_index] == board[row][col] {
                    temp_column[write_index] *= 2;
                    if write_index > 0 { write_index -= 1; } // Move to the next position up
                } else {
                    if write_index > 0 { write_index -= 1; } // Move to the next position up
                    temp_column[write_index] = board[row][col];
                }
            }
        }

        // Step 2: Update the original column with new values
        for row in 0..4 {
            board[row][col] = temp_column[row];
        }
    }
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
}
fn move_right(board: &mut Board) {
    for row in board.iter_mut() {
        // Step 1: Shift all tiles to the right
        let mut last_non_zero = 3;
        for i in (0..4).rev() {
            if row[i] != 0 {
                row.swap(i, last_non_zero);
                if last_non_zero > 0 {
                    last_non_zero -= 1;
                }
            }
        }

        // Step 2: Merge tiles
        for i in (0..3).rev() {
            if row[i] == row[i + 1] && row[i] != 0 {
                row[i + 1] *= 2;
                row[i] = 0;
            }
        }

        // Step 3: Shift again after merging
        let mut last_non_zero = 3;
        for i in (0..4).rev() {
            if row[i] != 0 {
                row.swap(i, last_non_zero);
                if last_non_zero > 0 {
                    last_non_zero -= 1;
                }
            }
        }
    }
}

fn game_loop(board: &mut Board) -> Result<(), crossterm::ErrorKind> {
    while !is_game_over(board) {
        let old_board: Board = board.clone();
        match get_user_input()? {
            UserAction::MoveUp => move_up(board),
            UserAction::MoveDown => move_down(board),
            UserAction::MoveLeft => move_left(board),
            UserAction::MoveRight => move_right(board),
            UserAction::Quit => break,
            UserAction::None => continue,
        }
        if board_changed(board, &old_board) {
            add_random_tile(board);
        }
        println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
        print_board(board);
    }
    Ok(())
}

fn main() {
    let mut board = init_board();
    let mut stdout = io::stdout();
    if let Err(e) = execute!(stdout, crossterm::terminal::SetSize(15, 15)){
        println!("Error: {}", e);
      }
        print_board(&mut board);
        game_loop(&mut board);
}