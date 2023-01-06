use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub enum Cell {
    #[default]
    Unset,
    Set(bool),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Side {
    pub len: usize,
    pub constraints: Vec<Vec<u32>>,
}

impl Side {
    fn new(len: usize) -> Side {
        Side {
            len,
            constraints: vec![vec![]; len],
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Board {
    pub x: Side,
    pub y: Side,
    pub state: Vec<Vec<Cell>>,
}

impl Board {
    pub fn default_state(x_len: usize, y_len: usize) -> Vec<Vec<Cell>> {
        vec![vec![Cell::default(); y_len]; x_len]
    }
    pub fn new(x_len: usize, y_len: usize) -> Board {
        Board {
            x: Side::new(x_len),
            y: Side::new(y_len),
            state: Self::default_state(x_len, y_len),
        }
    }
    pub fn by_sides(x: Side, y: Side) -> Board {
        Board {
            x: x.clone(),
            y: y.clone(),
            state: Self::default_state(x.len, y.len),
        }
    }
}
