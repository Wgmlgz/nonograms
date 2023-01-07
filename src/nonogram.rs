use serde::{Deserialize, Serialize};
use gloo_console::log;
use serde_wasm_bindgen::to_value;

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

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
struct SolveState {
    check_idx: u64,

}

impl Default for SolveState {
    fn default() -> SolveState {
        SolveState { check_idx:0 }
    }
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Board {
    pub x: Side,
    pub y: Side,
    pub state: Vec<Vec<Cell>>,
    solve_state: Option<SolveState>,
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
            solve_state: None
        }
    }
    pub fn by_sides(x: Side, y: Side) -> Board {
        Board {
            x: x.clone(),
            y: y.clone(),
            state: Self::default_state(x.len, y.len),
            solve_state: None
        }
    }

    fn gen(stack: &mut Vec<u32>) {
        stack.push(2);
    }

    fn solve_line(constraints: &Vec<u32>, old: &mut Vec<Cell>) {
        let len = old.len() as u32;
        let gaps = len - constraints.iter().sum::<u32>();
        assert!(gaps < len);

        let mut gap_stack: Vec<u32> = vec![];
        log!(to_value(&gap_stack).unwrap());

        Self::gen(&mut gap_stack);
        Self::gen(&mut gap_stack);
    }

    pub fn next(&mut self) -> Option<()> {
        if let Some(SolveState { check_idx }) = &mut self.solve_state {
            *check_idx += 1;
            *check_idx %= (self.x.len + self.y.len) as u64;
            log!(*check_idx);
            Self::solve_line(&self.x.constraints[0], &mut self.state[0]);
            if *check_idx < self.x.len as u64 {
                Some(());
            } else {
                Some(());
            }
            
        } else {
            self.solve_state = Some(SolveState::default());
        }
        Some(())
    }
}
