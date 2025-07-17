use crate::cell::Cell;
use crate::line::{Line, LineType};
use crate::puzzle::Puzzle;
use serde::{Deserialize, Serialize};
use std::error;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct PuzzleConfig {
    pub title: String,
    pub by: Option<String>,
    pub license: Option<String>,
    pub rows: Vec<Vec<u8>>,
    pub columns: Vec<Vec<u8>>,
}

impl PuzzleConfig {
    pub fn build(file_path: String) -> Result<Puzzle> {
        let path = std::path::Path::new(&file_path);
        let display = path.display();
        let file = match std::fs::File::open(&path) {
            Ok(file) => {
                log::debug!("Opened file OK");
                file
            }
            Err(err) => {
                eprintln!("Couldn't open {}: {}", display, err);
                return Err(err.into());
            }
        };
        let deserialized_puzzle: PuzzleConfig = match serde_yaml::from_reader(file) {
            Ok(puzzle_config) => {
                log::debug!("Loaded puzzle from file OK");
                puzzle_config
            }
            Err(err) => {
                eprintln!("Error deserializing YAML: {}", err);
                return Err(err.into());
            }
        };
        let row_count = deserialized_puzzle.rows.len() as u8;
        let col_count = deserialized_puzzle.columns.len() as u8;
        let row_clues = deserialized_puzzle.rows.clone();
        let col_clues = deserialized_puzzle.columns.clone();
        let mut rows = Vec::new();
        log::debug!("Generating row lines...");
        for i in 0..row_count {
            let line = Line::new(
                LineType::Row,
                i,
                vec![Cell::Unknown; col_count as usize],
                row_clues[i as usize].clone(),
            );
            log::debug!(
                "Generated {} which has {} potential solutions",
                line.label(),
                line.potential_solutions.len()
            );
            rows.push(line);
        }
        log::debug!("Generating col lines...");
        let mut cols = Vec::new();
        for i in 0..col_count {
            let line = Line::new(
                LineType::Col,
                i,
                vec![Cell::Unknown; row_count as usize],
                col_clues[i as usize].clone(),
            );
            log::debug!(
                "Generated {} which has {} potential solutions",
                line.label(),
                line.potential_solutions.len()
            );
            cols.push(line)
        }
        log::debug!("Row & col lines generated OK");
        let mut largest_col_clue_num = 0;
        for col_clue in &deserialized_puzzle.columns {
            for clue in col_clue {
                if *clue > largest_col_clue_num {
                    largest_col_clue_num = *clue;
                }
            }
        }
        let padding = (largest_col_clue_num.to_string().len() + 1) as u8;
        let author = if let Some(by) = deserialized_puzzle.by {
            by
        } else {
            String::from("")
        };
        let license = if let Some(license) = deserialized_puzzle.license {
            license
        } else {
            String::from("")
        };
        let new_puzzle = Puzzle {
            title: deserialized_puzzle.title,
            author,
            license,
            row_clues: deserialized_puzzle.rows.clone(),
            col_clues: deserialized_puzzle.columns.clone(),
            padding,
            rows,
            cols,
        };
        Ok(new_puzzle)
    }
}
