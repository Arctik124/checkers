
use colored::*;
use std::fmt::Display;

#[derive(Debug)]
pub struct Game {
    pub is_over: bool,
    pub winner: Option<String>,
    pub player_w: String,
    pub player_b: String,
    pub board: Board,
}

#[derive(Debug)]
pub struct Board {
    cells: [[Cell; 8]; 8],
}

impl Board {
    pub fn new() -> Self {
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
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let colored_text = "[ ][A][B][C][D][E][F][G][H][ ]".green();
        writeln!(f, "{}", colored_text);
        for (i, row) in self.cells.iter().enumerate() {
            let index = format!("[{}]", i + 1).green();
            write!(f, "{}", index);
            for col in row {
                write!(f, "{}", col);
            }
            write!(f, "{}", index);
            write!(f, "\n");
        }
        writeln!(f, "{}", colored_text);

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Cell {
    Empty,
    White,
    Black,
    WhiteQueen,
    BlackQueen,
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
            Cell::WhiteQueen => write!(
                f,
                "{}",
                ColoredString::from("[W]").black().on_bright_yellow()
            ),
            Cell::BlackQueen => write!(
                f,
                "{}",
                ColoredString::from("[B]").black().on_bright_yellow()
            ),
        }
    }
}
