use crate::cell::Cell;
use crate::line::{Line, LineType};
use colored::Colorize;

#[derive(Debug)]
pub struct Puzzle {
    pub title: String,
    pub author: String,
    pub license: String,
    pub row_clues: Vec<Vec<u8>>,
    pub col_clues: Vec<Vec<u8>>,
    pub rows: Vec<Line>,
    pub cols: Vec<Line>,
    pub padding: u8,
}

impl Puzzle {
    pub fn solve(&mut self, max_iterations: u32) -> (u32, bool) {
        let mut iterations: u32 = 0; //plenty of room for iterations :-)
        let mut progress_was_made = true;
        let mut puzzle_is_solved = false;
        while progress_was_made && iterations < max_iterations {
            puzzle_is_solved = true;
            progress_was_made = false;
            iterations += 1;
            log::debug!(
                "Starting iteration {}, with {} total potential individual line solutions remaining",
                iterations,
                self.total_potential_solutions_remaining()
            );
            for i in 0..self.row_count() {
                let mut line = self.get_line(LineType::Row, i);
                let (line_is_solved, progress) = line.solve();
                if line_is_solved == false {
                    puzzle_is_solved = false;
                }
                if progress == true {
                    progress_was_made = true;
                }
                self.set_line(line);
            }
            for i in 0..self.col_count() {
                let mut line = self.get_line(LineType::Col, i);
                let (line_is_solved, progress) = line.solve();
                if line_is_solved == false {
                    puzzle_is_solved = false;
                }
                if progress == true {
                    progress_was_made = true;
                }
                self.set_line(line);
            }
        }
        (iterations, puzzle_is_solved)
    }

    pub fn get_line(&self, axis: LineType, index: u8) -> Line {
        match axis {
            LineType::Row => self.rows[index as usize].clone(),
            LineType::Col => self.cols[index as usize].clone(),
        }
    }

    pub fn total_potential_solutions_remaining(&self) -> u32 {
        let mut total: u32 = 0;
        for line in &self.rows {
            total += line.potential_solutions.len() as u32;
        }
        for line in &self.cols {
            total += line.potential_solutions.len() as u32;
        }
        total
    }

    pub fn set_line(&mut self, line: Line) {
        match line.axis {
            LineType::Row => {
                self.rows[line.index as usize] = line.clone();
                for (i, cell) in line.cells.iter().enumerate() {
                    match cell {
                        Cell::Block => {
                            let mut col_line = self.cols[i].clone();
                            col_line.set_cell(line.index as u8, Cell::Block);
                            self.cols[i] = col_line.clone();
                        }
                        Cell::Space => {
                            let mut col_line = self.cols[i].clone();
                            col_line.set_cell(line.index as u8, Cell::Space);
                            self.cols[i] = col_line.clone();
                        }
                        _ => (),
                    }
                }
            }
            LineType::Col => {
                self.cols[line.index as usize] = line.clone();
                for (i, cell) in line.cells.iter().enumerate() {
                    match cell {
                        Cell::Block => {
                            let mut row_line = self.rows[i].clone();
                            row_line.set_cell(line.index as u8, Cell::Block);
                            self.rows[i] = row_line.clone();
                        }
                        Cell::Space => {
                            let mut row_line = self.rows[i].clone();
                            row_line.set_cell(line.index as u8, Cell::Space);
                            self.rows[i] = row_line.clone();
                        }
                        _ => (),
                    }
                }
            }
        }
    }

    pub fn row_count(&self) -> u8 {
        self.rows.len() as u8
    }

    pub fn col_count(&self) -> u8 {
        self.cols.len() as u8
    }

    fn max_col_clue_length(&self) -> u8 {
        let mut max_length = 0;

        for clue in &self.col_clues {
            if clue.len() > max_length {
                max_length = clue.len();
            }
        }
        max_length as u8
    }

    pub fn dump(&self) {
        println!("\nTitle: {}", self.title);
        if !self.author.is_empty() {
            println!("Author: {}", self.author);
        }

        for i in 0..(self.max_col_clue_length()) {
            let mut display_col_clues = String::new();
            for clue in self.col_clues.iter() {
                if clue.len() as u8 >= i + 1 {
                    display_col_clues += &format!(
                        "{:>width$} ",
                        clue[i as usize],
                        width = self.padding as usize
                    );
                } else {
                    display_col_clues += &format!("{:>width$} ", "", width = self.padding as usize);
                }
            }
            print!("{}", display_col_clues.yellow());
            println!("");
        }
        let rows = &self.rows.clone();
        for (line_index, line) in rows.iter().enumerate() {
            let mut display_cells = String::new();
            display_cells += " ";
            for cell in line.cells.iter() {
                display_cells +=
                    &format!("{:>width$} ", cell.display(), width = self.padding as usize);
            }
            print!("{}", display_cells.blue());
            display_cells = format!(" {}", clue_as_string(&self.row_clues[line_index]));
            print!("{}", display_cells.yellow());
            println!("");
        }
    }
}

pub fn clue_as_string(clue: &Vec<u8>) -> String {
    let mut clue_string = String::new();
    for (i, c) in clue.iter().enumerate() {
        if i > 0 {
            clue_string += ", ";
        }
        clue_string += &c.to_string();
    }
    clue_string
}
