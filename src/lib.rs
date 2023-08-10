mod utils;

use std::fmt::Display;
use wasm_bindgen::prelude::*;

pub use utils::set_panic_hook;

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Vec<Cell>>,
}

impl Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let (y, x) = (y as usize, x as usize);
                let symbol = if self.cells[y][x] == Cell::Dead {
                    '◻'
                } else {
                    '◼'
                };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn new(width: u32, height: u32) -> Self {
        let cells = (0..height)
            .map(|y| {
                (0..width)
                    .map(|x| {
                        let res = y * width + x;
                        if res % 2 == 0 || res % 7 == 0 {
                            Cell::Alive
                        } else {
                            Cell::Dead
                        }
                    })
                    .collect()
            })
            .collect();

        Self {
            width,
            height,
            cells,
        }
    }

    pub fn next_tick(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let (y, x) = (y as usize, x as usize);

                match (&self.cells[y][x], Self::cell_alive_neighbors(&self, y, x)) {
                    (&Cell::Alive, 2) | (&Cell::Alive, 3) => {}
                    (&Cell::Alive, _) => self.cells[y][x] = Cell::Dead,
                    (&Cell::Dead, 3) => self.cells[y][x] = Cell::Alive,
                    (&Cell::Dead, _) => {}
                }
            }
        }
    }

    pub fn render(&mut self) -> String {
        self.to_string()
    }

    fn cell_alive_neighbors(&self, y: usize, x: usize) -> u32 {
        let mut count = 0;

        let y_min = y.saturating_sub(1);
        let y_max = std::cmp::min(y + 1, self.height as usize - 1);
        let x_min = x.saturating_sub(1);
        let x_max = std::cmp::min(x + 1, self.width as usize - 1);

        for i in y_min..=y_max {
            for j in x_min..=x_max {
                match self.cells[i][j] {
                    Cell::Alive => count += 1,
                    Cell::Dead => {}
                }
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_saturating_remove() {
        assert_eq!(0_usize.saturating_sub(5), 0);
    }

    #[test]
    fn test_create_universe() {
        let universe = Universe::new(4, 4);

        assert_eq!(
            universe.cells,
            [
                [Cell::Alive, Cell::Dead, Cell::Alive, Cell::Dead],
                [Cell::Alive, Cell::Dead, Cell::Alive, Cell::Alive],
                [Cell::Alive, Cell::Dead, Cell::Alive, Cell::Dead],
                [Cell::Alive, Cell::Dead, Cell::Alive, Cell::Dead]
            ]
        );
    }

    #[test]
    fn test_universe_tick() {
        let mut universe = Universe::new(4, 4);

        universe.next_tick();
        universe.next_tick();
        universe.next_tick();
    }
}
