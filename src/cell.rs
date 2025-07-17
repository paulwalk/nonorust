const CELL_DISPLAY_UNKNOWN: char = '?';
const CELL_DISPLAY_BLOCK: char = '◼';
const CELL_DISPLAY_SPACE: char = '∙';
const CELL_DISPLAY_PADDING: &str = " ";

#[derive(Clone,Debug,PartialEq, Eq)]
pub enum Cell {
    Block,
    Space,
    Unknown,
}

impl Cell {
    pub fn display(&self) -> String {
        match self {
            Cell::Block => String::from(CELL_DISPLAY_BLOCK) + CELL_DISPLAY_PADDING,
            Cell::Space => String::from(CELL_DISPLAY_SPACE) + CELL_DISPLAY_PADDING,
            Cell::Unknown => String::from(CELL_DISPLAY_UNKNOWN) + CELL_DISPLAY_PADDING,
        }
    }
}

pub fn generate_cell_vector(cell: Cell, count: u8) -> Vec<Cell> {
    let mut cell_vector = Vec::new();
    for _ in 0..count {
        cell_vector.push(cell.clone());
    }
    cell_vector
}

pub fn display_cell_vector(cell_vector: &Vec<Cell>) -> String {
    let mut cells_display = String::new();
    for cell in cell_vector {
        cells_display += &cell.display();
    }
    cells_display
}

pub fn cell_vector_contains_unknown(cell_vector: &Vec<Cell>) -> bool {
    for cell in cell_vector {
        if *cell == Cell::Unknown {
            return true;
        }
    }
    false
}