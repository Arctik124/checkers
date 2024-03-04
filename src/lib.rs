use colored::*;
use std::error::Error;
use std::fmt::Display;
use std::io;
use std::num;

#[derive(Debug)]
pub struct Game {
    pub is_over: bool,
    pub winner: Option<String>,
    pub player_w: String,
    pub player_b: String,
    pub board: Board,
}

impl Game {
    pub fn new(player1: String, player2: String) -> Game {
        Game {
            is_over: false,
            winner: None,
            player_b: player1,
            player_w: player2,
            board: Board::new(),
        }
    }
}

#[derive(Debug)]
pub struct Board {
    cells: [[Cell; 8]; 8],
}

impl Board {
    fn new() -> Self {
        let mut cells: [[Cell; 8]; 8] = [[Cell::Empty; 8]; 8];

        for i in 0..8 {
            for j in 0..8 {
                cells[i][j] = Self::initial_cell(i, j);
            }
        }

        Self { cells }
    }

    fn initial_cell(row: usize, column: usize) -> Cell {
        match (row, column) {
            (0, col) | (2, col) if col % 2 == 1 => Cell::Black,
            (1, col) if col % 2 == 0 => Cell::Black,
            (5, col) | (7, col) if col % 2 == 0 => Cell::White,
            (6, col) if col % 2 == 1 => Cell::White,
            _ => Cell::Empty,
        }
    }

    fn get_cell(&self, row: u8, col: u8) -> Cell {
        (self.cells[row as usize])[col as usize]
    }

    fn set_cell(&mut self, row: u8, col: u8, cell: Cell) {
        (self.cells[row as usize])[col as usize] = cell
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let colored_text = "[ ][A][B][C][D][E][F][G][H][ ]".green();
        writeln!(f, "{}", colored_text)?;
        for (i, row) in self.cells.iter().enumerate() {
            let index = format!("[{}]", i + 1).green();
            write!(f, "{}", index)?;
            for col in row {
                write!(f, "{}", col)?;
            }
            write!(f, "{}", index)?;
            write!(f, "\n")?;
        }
        writeln!(f, "{}", colored_text)?;

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    White,
    Black,
    // WhiteQueen,
    // BlackQueen,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Cell::Empty => write!(
                f,
                "{}",
                ColoredString::from("[ ]").black().on_bright_yellow()
            ),
            Cell::White => write!(
                f,
                "{}",
                ColoredString::from("[w]").black().on_bright_yellow()
            ),
            Cell::Black => write!(
                f,
                "{}",
                ColoredString::from("[b]").black().on_bright_yellow()
            ),
            // Cell::WhiteQueen => write!(
            //     f,
            //     "{}",
            //     ColoredString::from("[W]").black().on_bright_yellow()
            // ),
            // Cell::BlackQueen => write!(
            //     f,
            //     "{}",
            //     ColoredString::from("[B]").black().on_bright_yellow()
            // ),
        }
    }
}

#[derive(Debug)]
struct Move {
    initial_pos: Coords,
    target_pos: Coords,
}

#[derive(Debug, PartialEq)]
struct Coords {
    row: u8,
    column: u8,
}

pub fn run(mut game: Game) -> Result<(), Box<dyn Error>> {
    let mut is_white = true;
    while !game.is_over {
        println!("{}", game.board);
        println!("{} turn", if is_white { "White" } else { "Black" });
        let mut cur_move = String::new();
        io::stdin()
            .read_line(&mut cur_move)
            .expect("Failed to read line");
        let cur_move = parse_input(cur_move)?;
        let can_move = can_move(&game.board, &cur_move, &is_white);
        if let Err(error) = can_move {
            println!("Cant move: {}", error);
            continue;
        }

        make_move(&mut game.board, &cur_move)?;
        // check_queen_condition();
        // is_can_continue();
        // toggle_player();
        // check_win_condition();
        is_white = !is_white;
    }
    Ok(())
}

// fn can_move(board: &Board, proposed_move: &Move, is_white: &bool) -> Result<(), &'static str> {
//     let initial_pos = &proposed_move.initial_pos;
//     let target_pos = &proposed_move.target_pos;

//     let start = board.get_cell(initial_pos.row, initial_pos.column);
//     let end = board.get_cell(target_pos.row, target_pos.column);

//     match end {
//         Cell::Empty => {}
//         _ => return Err("non empty target cell"),
//     }
//     let d_row: i32 = initial_pos.row as i32 - target_pos.row as i32;
//     let d_column: i32 = initial_pos.column as i32 - target_pos.column as i32;
//     match start {
//         Cell::White => {
//             if *is_white {
//                 if d_row > 0 && d_row.abs_diff(d_column) == 0 {
//                     Ok(())
//                 } else {
//                     Err("cant move there!")
//                 }
//             } else {
//                 Err("selected white piece move, however its black turn")
//             }
//         }
//         Cell::Black => {
//             if !*is_white {
//                 if d_row < 0 && d_row.abs_diff(d_column) == 0 {
//                     Ok(())
//                 } else {
//                     Err("cant move there!")
//                 }
//             } else {
//                 Err("selected white piece move, however its black turn")
//             }
//         }

//         Cell::Empty => Err("initial cell is empty"),
//         // _ => return Err("cant move this"),
//     }

//     // if initial_pos.1 - target_pos.1 == 1 && initial_pos.0 - target_pos.0 == 1 {
//     // Ok(())
//     // } else {
//     //     Err("cannot move there")
//     // }
// }
//

fn can_move(board: &Board, proposed_move: &Move, is_white: &bool) -> Result<(), &'static str> {
    let initial_pos = &proposed_move.initial_pos;
    let target_pos = &proposed_move.target_pos;

    let start = board.get_cell(initial_pos.row, initial_pos.column);
    let end = board.get_cell(target_pos.row, target_pos.column);

    match (start, end) {
        (Cell::Empty, _) => Err("initial cell is empty"),
        (_, Cell::White | Cell::Black) => Err("non-empty target cell"),

        (Cell::White, _) if !*is_white => Err("selected white piece, but it's black's turn"),
        (Cell::Black, _) if *is_white => Err("selected black piece, but it's white's turn"),

        (Cell::White, _) | (Cell::Black, _) => {
            let d_row = initial_pos.row as i32 - target_pos.row as i32;
            let d_column = initial_pos.column as i32 - target_pos.column as i32;

            // Valid forward move
            if d_row.abs_diff(d_column) == 1
                && ((*is_white && d_row == -1) || (!is_white && d_row == 1))
            {
                Ok(())
            } else if d_row.abs_diff(d_column) == 2 {
                // Valid capture move (jumping over opponent piece)
                let captured_row = (initial_pos.row as i32) - (d_row / 2);
                let captured_column = (initial_pos.column as i32) - (d_column / 2);
                let captured_cell = board.get_cell(captured_row as u8, captured_column as u8);

                match (captured_cell, start, d_row) {
                    (Cell::Black, Cell::White, 2) | (Cell::White, Cell::Black, -2) => Ok(()),
                    _ => Err("invalid capture move"),
                }
            } else {
                Err("invalid move")
            }
        }
    }
}

fn make_move(board: &mut Board, proposed_move: &Move) -> Result<(), &'static str> {
    let initial_pos = &proposed_move.initial_pos;
    let target_pos = &proposed_move.target_pos;

    let start = board.get_cell(initial_pos.row, initial_pos.column);
    let end = board.get_cell(target_pos.row, target_pos.column);

    board.set_cell(target_pos.row, target_pos.column, start);
    board.set_cell(initial_pos.row, initial_pos.column, end);

    Ok(())
}

fn parse_input(input: String) -> Result<Move, &'static str> {
    let splitted: Vec<char> = input.to_lowercase().trim().chars().collect();
    if splitted.len() > 5 {
        return Err("input len is greater than allowed");
    }

    Ok(Move {
        initial_pos: Coords {
            row: char_to_index(splitted[1])?,
            column: char_to_index(splitted[0])?,
        },
        target_pos: Coords {
            row: char_to_index(splitted[4])?,
            column: char_to_index(splitted[3])?,
        },
    })
}

fn char_to_index(c: char) -> Result<u8, &'static str> {
    match c {
        'a'..='h' => Ok((c as u8) - ('a' as u8)),
        '1'..='8' => Ok((c.to_digit(10).unwrap() - 1) as u8),
        _ => Err("cannot convert char {c} to index"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input_test() {
        let input = "a1 a3";
        let result = parse_input(input.to_string()).unwrap();
        let target = Move {
            initial_pos: Coords { row: 0, column: 0 },
            target_pos: Coords { row: 2, column: 0 },
        };
        assert_eq!(target.initial_pos, result.initial_pos);
        assert_eq!(target.target_pos, result.target_pos);
    }

    #[test]
    fn char_to_index_test() {
        let c = 'a';
        assert_eq!(0, char_to_index(c).unwrap())
    }

    #[test]
    fn can_move_test_black() {
        let board = Board::new();
        let proposed_move = Move {
            initial_pos: Coords { row: 2, column: 1 },
            target_pos: Coords { row: 3, column: 0 },
        };
        let result = can_move(&board, &proposed_move, &false);
        assert_eq!(Ok(()), result);
    }

    #[test]
    fn can_move_test_err() {
        let board = Board::new();
        // b3 b4
        let proposed_move = Move {
            initial_pos: Coords { row: 2, column: 1 },
            target_pos: Coords { row: 3, column: 1 },
        };
        let result = can_move(&board, &proposed_move, &false);
        assert_eq!(Err("cant move there!"), result);
    }

    #[test]
    fn can_move_test_white() {
        let board = Board::new();
        let proposed_move = Move {
            initial_pos: Coords { row: 5, column: 2 },
            target_pos: Coords { row: 4, column: 1 },
        };
        let result = can_move(&board, &proposed_move, &true);
        assert_eq!(Ok(()), result);
    }

    #[test]
    fn can_move_test_empty_start() {
        let board = Board::new();
        let proposed_move = Move {
            initial_pos: Coords { row: 5, column: 1 },
            target_pos: Coords { row: 4, column: 0 },
        };
        let result = can_move(&board, &proposed_move, &true);
        assert_eq!(Err("initial cell is empty"), result);
    }

    #[test]
    fn can_move_test_nonempty_finish() {
        let board = Board::new();
        let proposed_move = Move {
            initial_pos: Coords { row: 6, column: 1 },
            target_pos: Coords { row: 5, column: 2 },
        };
        let result = can_move(&board, &proposed_move, &true);
        assert_eq!(Err("non empty target cell"), result);
    }

    #[test]
    fn get_cell_test() {
        let board = Board::new();
        assert_eq!(Cell::Empty, board.get_cell(0, 0));
        assert_eq!(Cell::Black, board.get_cell(0, 1));
        assert_eq!(Cell::White, board.get_cell(5, 4));
        assert_eq!(Cell::White, board.get_cell(7, 0));
    }
}
