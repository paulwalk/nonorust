use crate::cell::{cell_vector_contains_unknown, Cell};
use crate::line_algorithms::{generate_all_potential_solutions_for_clue, factorial};
use colored::Colorize;

#[derive(Debug, Clone)]

pub enum LineType {
    Row,
    Col,
}

#[derive(Debug, Clone)]
pub struct Line {
    pub axis: LineType,
    pub index: u8,
    pub clue: Vec<u8>,
    pub cells: Vec<Cell>,
    pub potential_solutions: Vec<Vec<Cell>>,
}

impl Line {
    pub fn new(axis: LineType, index: u8, cells: Vec<Cell>, clue: Vec<u8>) -> Line {

        let potential_solutions =
            generate_all_potential_solutions_for_clue(clue.clone(), cells.len() as i8);
        let new_line = Line {
            axis,
            index,
            clue,
            cells,
            potential_solutions,
        };
        new_line
    }

    pub fn length(&self) -> u8 {
        self.cells.len() as u8
    }

    pub fn get_cell(&self, index: u8) -> Cell {
        self.cells[index as usize].clone()
    }

    pub fn set_cell(&mut self, index: u8, cell: Cell) {
        self.cells[index as usize] = cell;
    }

    pub fn label(&self) -> String {
        match self.axis {
            LineType::Row => format!("Row {}", self.index + 1),
            LineType::Col => format!("Col {}", self.index + 1),
        }
    }

    pub fn solve(&mut self) -> (bool, bool) {
        let line_solved: bool;
        let mut progress_made: bool;
        if self.is_solved() {
            line_solved = true;
            progress_made = false;
            return (line_solved, progress_made)
        }

        if self.potential_solutions.len() > 0 && !cell_vector_contains_unknown(&self.cells) {
            line_solved = true;
            progress_made = true;
            self.potential_solutions.clear();
            return (line_solved, progress_made)
        }

        if self.potential_solutions.len() == 1 {
            let potential_solutions = self.potential_solutions.clone();
            for solution in potential_solutions {
                for i in 0..solution.len() {
                    self.set_cell(i as u8, solution[i].clone());
                }
            }
            self.potential_solutions.clear();
            // line_solved = false is intentional here, even though the line is solved at this point,
            // because we want it to be reported back to the puzzle solver as unsolved but with progress made,
            // so that the puzzle solver records this line's changes.
            line_solved = false;
            progress_made = true;
            return(line_solved, progress_made)
        } else {
            line_solved = false;
            progress_made = false;
            let progress_from_algorithm_1 =
                self.find_cells_which_are_same_in_all_potential_solutions();
            let progress_from_algorithm_2 = self.remove_solutions_which_do_not_fit_known_cells();
            if progress_from_algorithm_1 || progress_from_algorithm_2 {
                progress_made = true;
            }
            return (line_solved, progress_made)
        }
    }

    pub fn is_solved(&self) -> bool {
        self.potential_solutions.len() == 0
    }

    pub fn dump(&self) {
        let mut cells_display = String::new();
        for cell in &self.cells {
            cells_display += &cell.display();
        }
        println!(
            "{}:  Clue: {:?}  Length: {}  Potential Solutions Remaining: {}  Cells: {}",
            self.label(),
            self.clue,
            self.length(),
            self.potential_solutions.len(),
            cells_display.blue(),
        );
    }

    pub fn calculate_all_potential_solutions(&self) -> u128 {
        let sum_of_blocks:u128 = self.clue.iter().map(|&x| x as u128).sum();
        let n:u128 = (self.length() as u128 - sum_of_blocks) + 1;
        factorial(n) / (factorial(self.clue.len() as u128) * factorial(n - self.clue.len() as u128))
    }

    pub fn dump_potential_solutions(&self) {
        println!("Potential solutions for {}:", self.label());
        for solution in &self.potential_solutions {
            let mut solution_str = String::new();
            for cell in solution {
                solution_str += &cell.display()
            }
            println!("{}", solution_str.blue());
        }
    }
}
