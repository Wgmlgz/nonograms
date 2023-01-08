use gloo_console::log;
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;

#[derive(Clone, PartialEq, Copy, Debug, Default, Serialize, Deserialize)]
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
pub struct SolveState {
    pub counter: usize,
    pub is_x: bool,
    pub idx: usize,
}

impl Default for SolveState {
    fn default() -> SolveState {
        SolveState {
            counter: 0,
            is_x: true,
            idx: 0,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Board {
    pub x: Side,
    pub y: Side,
    pub state: Vec<Vec<Cell>>,
    pub solve_state: Option<SolveState>,
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
            solve_state: None,
        }
    }
    pub fn by_sides(x: Side, y: Side) -> Board {
        Board {
            x: x.clone(),
            y: y.clone(),
            state: Self::default_state(x.len, y.len),
            solve_state: None,
        }
    }

    pub fn next(&mut self) -> Option<()> {
        if let Some(SolveState { counter, is_x, idx }) = &mut self.solve_state {
            let c_idx = *counter;
            let use_x = c_idx < self.x.len;
            let c_idx = if use_x { c_idx } else { c_idx - self.x.len };

            *is_x = use_x;
            *idx = c_idx;

            let (constraints, state) = if use_x {
                (&self.x.constraints[c_idx], self.state[c_idx].clone())
            } else {
                let cashed = self
                    .state
                    .iter()
                    .map(|line| line[c_idx])
                    .collect::<Vec<_>>();
                (&self.y.constraints[c_idx], cashed)
            };

            // log!(c_idx);
            // log!("constraints", to_value(&constraints).unwrap());
            // log!("state", to_value(&state).unwrap());

            if let Some(updated) = solve_line(constraints, &state) {
                // log!("solved", to_value(&updated).unwrap());
                if use_x {
                    self.state[c_idx] = updated;
                } else {
                    for (pos, &cell) in updated.iter().enumerate() {
                        self.state[pos][c_idx] = cell;
                    }
                }
            } else {
                // log!("none(");
            }

            *counter += 1;
            *counter %= self.x.len + self.y.len;
        } else {
            self.solve_state = Some(SolveState::default());
        }
        Some(())
    }
}

/** Tries to solve line based on `constraints` and if can, return updated row */
fn solve_line(constraints: &Vec<u32>, old: &Vec<Cell>) -> Option<Vec<Cell>> {
    let len = old.len() as u32;
    let gaps = len - constraints.iter().sum::<u32>();

    // if gaps > (len - gaps) {
    //     return None;
    // }
    assert!(gaps < len);

    fn check_block(line: &Vec<Cell>, idx: u32, len: u32) -> bool {
        (idx..(idx + len)).all(|cur| match line.get(cur as usize) {
            Some(Cell::Set(true)) => true,
            Some(Cell::Unset) => true,
            _ => false,
        })
    }

    struct Env<'a> {
        old: &'a Vec<Cell>,
        constraints: &'a Vec<u32>,
        stack: &'a mut Vec<u32>,
        searched: Option<Vec<Cell>>,
    }

    fn dfs(env: &mut Env, begin: u32, cur: usize) {
        let len = env.old.len() as u32;

        if cur >= env.constraints.len() {
            let end_clear = ((begin - 1)..len).all(|cur| match env.old.get(cur as usize) {
                Some(Cell::Set(false) | Cell::Unset) => true,
                _ => false,
            });
            if end_clear {
                let mut v = vec![Cell::Set(false); len as usize];

                for (pos, &begins) in env.stack.iter().enumerate() {
                    for i in begins..(begins + env.constraints[pos]) {
                        v[i as usize] = Cell::Set(true);
                    }
                }

                if let Some(searched) = &mut env.searched {
                    searched.iter_mut().zip(v).for_each(|(old, new)| {
                        if *old != new {
                            *old = Cell::Unset;
                        }
                    });
                } else {
                    env.searched = Some(v);
                }
            }
            return;
        }

        let cur_len = env.constraints[cur];

        for idx in begin..(len - cur_len + 1) {
            let prev_match = idx == 0
                || match env.old.get((idx - 1) as usize) {
                    Some(Cell::Set(true)) => false,
                    _ => true,
                };
            if !prev_match {
                return;
            }
            env.stack.push(idx);
            if check_block(env.old, idx, cur_len) {
                dfs(env, idx + cur_len + 1, cur + 1);
            }
            env.stack.pop();
        }
    }

    let mut stack = vec![];
    let mut env = Env {
        old,
        constraints,
        stack: &mut stack,
        searched: None,
    };
    dfs(&mut env, 0, 0);

    log!("env.found");

    env.searched
}
