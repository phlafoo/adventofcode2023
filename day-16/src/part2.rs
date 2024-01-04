use crate::custom_error::AocError;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up = 0b1000,
    Down = 0b0100,
    Left = 0b0010,
    Right = 0b0001,
}

use Direction::*;

#[derive(Debug, Clone, Copy)]
struct Photon {
    index: usize,
    direction: Direction,
}

impl Photon {
    pub fn new(index: usize, direction: Direction) -> Photon {
        Photon { index, direction }
    }
}

struct BeamMap<'a> {
    grid: &'a [u8],
    width: usize,
    photons: Vec<Photon>,
    energized: Vec<u8>,
}

impl<'a> BeamMap<'a> {
    pub fn init(grid: &'a [u8]) -> BeamMap<'a> {
        let energized = vec![0; grid.len()];
        let width = grid.iter().position(|&t| t == b'\n').unwrap();

        BeamMap {
            grid,
            width,
            photons: vec![],
            energized,
        }
    }

    pub fn reset(&mut self, start: Photon) {
        self.energized.fill(0);
        self.photons = vec![start];
    }

    pub fn print_grid(&self) {
        for i in 0..self.energized.len() {
            if self.grid[i] != b'.' {
                print!("{}", self.grid[i] as char);
                continue;
            }
            match self.energized[i] {
                0 => print!("."),
                0b1000 => print!("▲"),
                0b0100 => print!("▼"),
                0b0010 => print!("◄"),
                0b0001 => print!("►"),
                mask => {
                    print!("{}", mask.count_ones());
                }
            }
        }
        println!();
    }

    pub fn get_energized_count(&self) -> u32 {
        self.energized
            .iter()
            .fold(0, |acc, &e| acc + (e != 0) as u32)
    }

    /// Returns false if direction was already set
    fn set_direction_at(&mut self, dir: Direction, index: usize) -> bool {
        let before = self.energized[index];
        self.energized[index] |= dir as u8;
        before != self.energized[index]
    }

    /// Get next direction based on current tile and direction. Also adds photon if split
    fn get_next_dir(&mut self, tile_index: usize, dir: Direction) -> Direction {
        let tile = self.grid[tile_index];
        match dir {
            Up => match tile {
                b'-' => {
                    self.add_photon(tile_index, Right);
                    Left
                }
                b'\\' => Left,
                b'/' => Right,
                _ => Up,
            },
            Down => match tile {
                b'-' => {
                    self.add_photon(tile_index, Right);
                    Left
                }
                b'\\' => Right,
                b'/' => Left,
                _ => Down,
            },
            Left => match tile {
                b'|' => {
                    self.add_photon(tile_index, Down);
                    Up
                }
                b'\\' => Up,
                b'/' => Down,
                _ => Left,
            },
            Right => match tile {
                b'|' => {
                    self.add_photon(tile_index, Down);
                    Up
                }
                b'\\' => Down,
                b'/' => Up,
                _ => Right,
            },
        }
    }

    /// Get next tile index based on new direction
    fn get_next_tile_index(&self, tile_index: usize, new_dir: Direction) -> Option<usize> {
        match new_dir {
            Up => {
                if tile_index > self.width {
                    Some(tile_index - self.width - 1)
                } else {
                    None
                }
            }
            Down => {
                let new_index = tile_index + self.width + 1;
                if new_index < self.grid.len() {
                    Some(new_index)
                } else {
                    None
                }
            }
            Left => {
                if tile_index % (self.width + 1) != 0 {
                    Some(tile_index - 1)
                } else {
                    None
                }
            }
            Right => {
                if (tile_index + 2) % (self.width + 1) != 0 {
                    Some(tile_index + 1)
                } else {
                    None
                }
            }
        }
    }

    fn add_photon(&mut self, tile_index: usize, dir: Direction) {
        if let Some(new_index) = self.get_next_tile_index(tile_index, dir) {
            self.photons.push(Photon::new(new_index, dir));
        }
    }

    fn update_photon(&mut self, photon_index: usize, new_dir: Direction) {
        match self.get_next_tile_index(self.photons[photon_index].index, new_dir) {
            Some(i) => {
                let p = &mut self.photons[photon_index];
                p.index = i;
                p.direction = new_dir;
            }
            None => {
                self.photons.remove(photon_index);
            }
        }
    }

    /// Advances all photons. Returns false if there are no photons left
    pub fn advance_photons(&mut self) -> bool {
        // Iterate in reverse because some photons may get removed and shift elements to its right
        for photon_index in (0..self.photons.len()).rev() {
            let p = self.photons[photon_index];

            // Cache direction at this tile
            if !self.set_direction_at(p.direction, p.index) {
                // Remove if already set
                self.photons.remove(photon_index);
                continue;
            }
            let new_dir = self.get_next_dir(p.index, p.direction);
            self.update_photon(photon_index, new_dir);
        }
        !self.photons.is_empty()
    }
}

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut beam_map = BeamMap::init(input.as_bytes());
    let grid_len = beam_map.grid.len();
    let width = beam_map.width;

    // Naively run simulation for each starting photon
    let up_iter = (grid_len - width..grid_len).map(|i| Photon::new(i, Up));

    let down_iter = (0..width).map(|i| Photon::new(i, Down));

    let left_iter = (width - 1..grid_len)
        .step_by(width + 1)
        .map(|i| Photon::new(i, Left));

    let right_iter = (0..grid_len)
        .step_by(width + 1)
        .map(|i| Photon::new(i, Right));

    let result = up_iter
        .chain(down_iter)
        .chain(left_iter)
        .chain(right_iter)
        .fold(0, |max, start| {
            beam_map.reset(start);

            while beam_map.advance_photons() {}

            max.max(beam_map.get_energized_count())
        });

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        assert_eq!("51", process(input)?);
        Ok(())
    }
}
