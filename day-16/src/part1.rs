use crate::custom_error::AocError;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
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
    energized: Vec<[bool; 4]>,
}

impl<'a> BeamMap<'a> {
    pub fn print_grid(&self) {
        for i in 0..self.energized.len() {
            if self.grid[i] != b'.' {
                print!("{}", self.grid[i] as char);
                continue;
            }
            match self.energized[i] {
                [false, false, false, false] => print!("."),
                [true, false, false, false] => print!("▲"),
                [false, true, false, false] => print!("▼"),
                [false, false, true, false] => print!("◄"),
                [false, false, false, true] => print!("►"),
                mask => {
                    let count = mask.iter().filter(|&&b| b).count();
                    print!("{count}");
                }
            }
        }
        println!();
    }

    pub fn get_energized_count(&self) -> usize {
        self.energized
            .iter()
            .fold(0, |acc, e| if e.contains(&true) { acc + 1 } else { acc })
    }

    pub fn init(grid: &'a [u8]) -> BeamMap<'a> {
        // [up, down, left, right]
        let energized = vec![Default::default(); grid.len()];
        let width = grid.iter().position(|&t| t == b'\n').unwrap();

        BeamMap {
            grid,
            width,
            photons: vec![Photon::new(0, Right)],
            energized,
        }
    }

    /// Returns false if direction was already set
    fn set_direction_at(&mut self, dir: Direction, index: usize) -> bool {
        let has_dir = &mut self.energized[index][dir as usize];
        if *has_dir {
            false
        } else {
            *has_dir = true;
            true
        }
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

    while beam_map.advance_photons() {}

    let result = beam_map.get_energized_count();

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
        assert_eq!("46", process(input)?);
        Ok(())
    }
}
