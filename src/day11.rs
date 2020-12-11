use std::cmp::min;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Tile {
    Floor,
    Empty,
    Occupied,
}

#[derive(Clone, PartialEq, Debug)]
struct SeatGrid {
    map: Vec<Tile>,
    cols: usize,
    rows: usize,
}

impl SeatGrid {
    fn get(&self, column: usize, row: usize) -> Option<Tile> {
        let column = column % self.cols;
        if row < self.rows {
            Some(self.map[row * self.cols + column])
        } else {
            None
        }
    }

    fn num_neighbors_occupied(&self, column: usize, row: usize) -> usize {
        let mut count = 0;
        for r in row.saturating_sub(1)..=min(row+1, self.rows - 1) {
            for c in column.saturating_sub(1)..=min(column+1, self.cols - 1) {
                if c == column && r == row {
                    continue;
                }
                match self.get(c, r) {
                    Some(Tile::Occupied) => count += 1,
                    Some(_) => {},
                    None => panic!("We messed up the math"),
                }
            }
        }
        count
    }

    fn num_neighbors_visible(&self, column: usize, row: usize) -> usize {
        let direction_vectors = 
[(-1,-1), (-1, 0), (-1, 1),
 ( 0,-1),          ( 0, 1),
 ( 1,-1), ( 1, 0), ( 1, 1)];

        let mut count = 0;
        for (dr, dc) in direction_vectors.iter() {
            let mut row = row as i32 + dr;
            let mut column = column as i32 + dc;
            while 0 <= row && row < self.rows as i32 &&
                  0 <= column && column < self.cols as i32
            {
                match self.get(column as usize, row as usize).unwrap() {
                    Tile::Floor => {}
                    Tile::Empty => break,
                    Tile::Occupied => {
                        count += 1;
                        break;
                    }
                }
                row += dr;
                column += dc;
            }
        }

        count
    }

    fn count_total_seated(&self) -> usize {
        let mut count = 0;
        for tile in &self.map {
            match tile {
                Tile::Occupied => count += 1,
                _ => {},
            }
        }
        count
    }

    fn update_part1(&mut self) -> bool {
        let mut new_map = vec![];
        for r in 0..self.rows {
            for c in 0..self.cols {
                match self.get(c, r).expect("bad math") {
                    Tile::Floor => new_map.push(Tile::Floor),
                    Tile::Empty => {
                        if self.num_neighbors_occupied(c, r) == 0 {
                            new_map.push(Tile::Occupied);
                        } else {
                            new_map.push(Tile::Empty);
                        }
                    },
                    Tile::Occupied => {
                        if self.num_neighbors_occupied(c, r) >= 4 {
                            new_map.push(Tile::Empty);
                        } else {
                            new_map.push(Tile::Occupied);
                        }
                    },
                }
            }
        }

        let changed = new_map != self.map;
        self.map = new_map;
        changed
    }

    fn update_part2(&mut self) -> bool {
        let mut new_map = vec![];
        for r in 0..self.rows {
            for c in 0..self.cols {
                match self.get(c, r).expect("bad math") {
                    Tile::Floor => new_map.push(Tile::Floor),
                    Tile::Empty => {
                        if self.num_neighbors_visible(c, r) == 0 {
                            new_map.push(Tile::Occupied);
                        } else {
                            new_map.push(Tile::Empty);
                        }
                    },
                    Tile::Occupied => {
                        if self.num_neighbors_visible(c, r) >= 5 {
                            new_map.push(Tile::Empty);
                        } else {
                            new_map.push(Tile::Occupied);
                        }
                    },
                }
            }
        }

        let changed = new_map != self.map;
        self.map = new_map;
        changed
    }
}

fn parse_grid(input: &str) -> SeatGrid {
    let mut map = vec![];
    let mut rows = 0;
    let mut cols = None;

    for line in input.lines() {
        if let Some(cols) = cols {
            assert_eq!(line.len(), cols);
        } else {
            cols = Some(line.len());
        }
        rows += 1;
        
        for b in line.bytes() {
            match b {
                b'.' => map.push(Tile::Floor),
                b'L' => map.push(Tile::Empty),
                b'#' => map.push(Tile::Occupied),
                unknown_char => panic!("Unrecognized character: `{}`", unknown_char as char),
            }
        }
    }

    let cols = cols.unwrap();

    SeatGrid {
        map,
        cols,
        rows,
    }
}

#[aoc(day11, part1)]
fn solve_part1(input: &str) -> usize {
    let mut map = parse_grid(input);
    while map.update_part1() {}
    map.count_total_seated()
}

#[aoc(day11, part2)]
fn solve_part2(input: &str) -> usize {
    let mut map = parse_grid(input);
    while map.update_part2() {}
    map.count_total_seated()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_INPUT: &str =
"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn test_example_part1() {
        let output = solve_part1(EXAMPLE_INPUT);
        assert_eq!(output, 37);
    }

    #[test]
    fn test_example_part2() {
        let output = solve_part2(EXAMPLE_INPUT);
        assert_eq!(output, 26);
    }
}