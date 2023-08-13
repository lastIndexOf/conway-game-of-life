mod utils;

use std::{fmt::Display, iter::repeat};
use wasm_bindgen::prelude::*;

pub use utils::set_panic_hook;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn info(s: &str);
}

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

    pub fn from_vec(arr: Vec<u8>, width: u32, height: u32) -> Self {
        assert!(width > 0);
        assert!(height > 0);

        let mut cells = repeat(Vec::with_capacity(width as usize))
            .take(height as usize)
            .collect::<Vec<_>>();

        for y in 0..height {
            for x in 0..width {
                let (y, x) = (y as usize, x as usize);
                let index = y * width as usize + x;
                cells[y].push(match arr[index] {
                    0 => Cell::Dead,
                    1 => Cell::Alive,
                    _ => panic!("Invalid cell state"),
                });
            }
        }

        Self {
            width,
            height,
            cells,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self, height: usize) -> *const Cell {
        self.cells[height].as_ptr()
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..height)
            .map(|_| (0..self.width).map(|_| Cell::Dead).collect())
            .collect();
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..self.height)
            .map(|_| (0..width).map(|_| Cell::Dead).collect())
            .collect();
    }

    pub fn next_tick(&mut self) {
        let mut cells = self.cells.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                let (y, x) = (y as usize, x as usize);

                match (&self.cells[y][x], Self::cell_alive_neighbors(&self, y, x)) {
                    (&Cell::Alive, 2) | (&Cell::Alive, 3) => {}
                    (&Cell::Dead, 3) => cells[y][x] = Cell::Alive,
                    (&Cell::Alive, _) => cells[y][x] = Cell::Dead,
                    (&Cell::Dead, _) => {}
                }
            }
        }

        self.cells = cells;
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    fn cell_alive_neighbors(&self, y: usize, x: usize) -> usize {
        let mut count = 0;

        for dy in -1..=1 {
            for dx in -1..=1 {
                if dy == 0 && dx == 0 {
                    continue;
                }

                let (y, x) = (y as i32, x as i32);
                let (height, width) = (self.height as i32, self.width as i32);

                let new_y = (((dy + y) % height) + height) % height;
                let new_x = (((dx + x) % width) + width) % width;

                match self.cells[new_y as usize][new_x as usize] {
                    Cell::Alive => count += 1,
                    Cell::Dead => {}
                };
            }
        }

        count
    }
}

impl Universe {
    pub fn get_cells(&self) -> &Vec<Vec<Cell>> {
        &self.cells
    }

    pub fn set_cells(&mut self, cells: &[(usize, usize)]) {
        for &(y, x) in cells {
            self.cells[y][x] = Cell::Alive;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_saturating_remove() {
        assert_eq!(0_usize.saturating_sub(5), 0);
        assert_eq!(-1_i32 % 64, -1);
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
