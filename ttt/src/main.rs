use std::io;
use std::process::Command;

fn print_board(board: &[[char; 3]; 3]) {
    println!("-------------");
    for row in board.iter() {
        println!("| {} | {} | {} |", row[0], row[1], row[2]);
        println!("-------------");
    }
}

fn check_win(board: &[[char; 3]; 3]) -> Option<char> {
    for i in 0..3 {
        if board[i][0] == board[i][1] && board[i][1] == board[i][2] && board[i][0] != ' ' {
            return Some(board[i][0]);
        }
        if board[0][i] == board[1][i] && board[1][i] == board[2][i] && board[0][i] != ' ' {
            return Some(board[0][i]);
        }
    }
    if board[0][0] == board[1][1] && board[1][1] == board[2][2] && board[0][0] != ' ' {
        return Some(board[0][0]);
    }
    if board[0][2] == board[1][1] && board[1][1] == board[2][0] && board[0][2] != ' ' {
        return Some(board[0][2]);
    }
    None
}

fn main() {
    let mut board = [[' '; 3]; 3];
    let mut player = 'X';
    loop {
        print_board(&board);
        println!(
            "{}'s turn. Enter row and column separated by space (e.g. '1 2'): ",
            player
        );
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
        let coords: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let row = coords[0] - 1;
        let col = coords[1] - 1;
        if board[row][col] == ' ' {
            board[row][col] = player;
            match check_win(&board) {
                Some(winner) => {
                    print_board(&board);
                    println!("{} wins!", winner);
                    break;
                }
                None => {
                    if board.iter().all(|row| row.iter().all(|&cell| cell != ' ')) {
                        print_board(&board);
                        println!("Tie!");
                        break;
                    }
                    player = if player == 'X' { 'O' } else { 'X' };
                }
            }
        } else {
            println!("That space is already taken.");
        }
        if cfg!(target_os = "windows") {
            Command::new("cls").status().unwrap();
        } else {
            Command::new("clear").status().unwrap();
        };
    }
}
