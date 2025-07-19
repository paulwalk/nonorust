use crate::cell::{Cell, generate_cell_vector};
use crate::line::Line;

pub fn factorial(number: u128) -> u128 {
    let mut factorial: u128 = 1;
    for i in 1..(number + 1) {
        println!("i = {}, number = {}", i, number);
        factorial *= i;
    }
    factorial
}

pub fn generate_all_potential_solutions_for_clue(clue: Vec<u8>, length: i8) -> Vec<Vec<Cell>> {
    if clue.len() == 0 {
        let mut solutions: Vec<Vec<Cell>> = Vec::new();
        let cell_vector = generate_cell_vector(Cell::Space, length as u8);
        solutions.push(cell_vector);
        solutions
    } else {
        let starts = length - clue[0] as i8;
        if clue.len() == 1 {
            let mut solutions: Vec<Vec<Cell>> = Vec::new();
            for i in 0..starts + 1 {
                let mut solution: Vec<Cell> = Vec::new();
                solution.append(&mut generate_cell_vector(Cell::Space, i as u8));
                solution.append(&mut generate_cell_vector(Cell::Block, clue[0]));
                solution.append(&mut generate_cell_vector(Cell::Space, (starts - i) as u8));
                solutions.push(solution);
            }
            solutions
        } else {
            let mut solutions: Vec<Vec<Cell>> = Vec::new();
            for i in 0..starts {
                for j in
                    generate_all_potential_solutions_for_clue(clue[1..].to_owned(), starts - i - 1)
                {
                    let mut solution: Vec<Cell> = Vec::new();
                    solution.append(&mut generate_cell_vector(Cell::Space, i as u8));
                    solution.append(&mut generate_cell_vector(Cell::Block, clue[0]));
                    solution.push(Cell::Space);
                    solution.append(&mut j.clone());
                    solutions.push(solution);
                }
            }
            solutions
        }
    }
}

impl Line {
    pub fn find_cells_which_are_same_in_all_potential_solutions(&mut self) -> bool {
        let mut progress_made = false;
        let cells = &self.cells.clone();
        for (cell_index, cell) in cells.iter().enumerate() {
            if cell == &Cell::Unknown {
                let mut found_non_block_cell = false;
                for solution in self.potential_solutions.iter() {
                    if solution[cell_index] != Cell::Block {
                        found_non_block_cell = true;
                        break;
                    }
                }
                if found_non_block_cell == false {
                    self.set_cell(cell_index as u8, Cell::Block);
                    progress_made = true;
                } else {
                    let mut found_non_space_cell = false;
                    for solution in self.potential_solutions.iter() {
                        if solution[cell_index] != Cell::Space {
                            found_non_space_cell = true;
                            break;
                        }
                    }
                    if found_non_space_cell == false {
                        self.set_cell(cell_index as u8, Cell::Space);
                        progress_made = true;
                    }
                }
            }
        }
        progress_made
    }

    // pub fn remove_solutions_which_do_not_fit_known_cells(&mut self) -> bool {
    //     let mut progress_made = false;
    //     self.potential_solutions.retain(|solution| {
    //         let mut retain = true;
    //         for (cell_index, cell) in self.cells.iter().enumerate() {
    //             if cell != &Cell::Unknown && solution[cell_index] != *cell {
    //                 progress_made = true;
    //                 retain = false;
    //                 break;
    //             }
    //         }
    //         retain
    //     });
    //     progress_made
    // }

    pub fn remove_solutions_which_do_not_fit_known_cells(&mut self) -> bool {
        let mut progress_made = false;
        let cells = &self.cells.clone();
        for (cell_index, cell) in cells.iter().enumerate() {
            if cell != &Cell::Unknown {
                self.potential_solutions.retain(|solution| {
                    if solution[cell_index] == *cell {
                        true
                    } else {
                        progress_made = true;
                        false
                    }
                });
            }
        }
        progress_made
    }
}
