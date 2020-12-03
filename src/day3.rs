#[aoc_generator(day3)]
fn generate_map(input: &str) -> Map {
    let mut map = vec![];
    let mut line_length = 0;
    let mut rows = 0;
    for line in input.lines() {
        let this_line_length = line.len();
        if line_length == 0 {
            line_length = this_line_length;
        } else if line_length != this_line_length {
            panic!("Bad input, line lengths not constant");
        }

        map.extend(line.as_bytes()
            .iter()
            .map(|b| {
                match b {
                    b'.' => SlopeObject::Empty,
                    b'#' => SlopeObject::Tree,
                    unknown_byte => panic!("Unrecognized byte: `{}`", *unknown_byte as char),
                }
            })
        );
        rows += 1;
    }

    assert!(line_length != 0);
    Map {
        map,
        line_length,
        rows,
    }
}

#[derive(Copy, Clone)]
enum SlopeObject {
    Empty,
    Tree,
}

struct Map {
    map: Vec<SlopeObject>,
    line_length: usize,
    rows: usize,
}

impl Map {
    fn get(&self, row: usize, column: usize) -> Option<SlopeObject> {
        let column = column % self.line_length;
        if row < self.rows {
            Some(self.map[row * self.line_length + column])
        } else {
            None
        }
    }
}

fn num_trees_hit(input: &Map, delta_col: usize, delta_row: usize) -> usize {
    let mut pos_row = 0;
    let mut pos_col = 0;
    let mut num_trees = 0;

    while let Some(obj) = input.get(pos_row, pos_col) {
        match obj {
            SlopeObject::Tree => num_trees += 1,
            _ => {},
        }
        pos_row += delta_row;
        pos_col += delta_col;
    }

    num_trees
}
#[aoc(day3, part1)]
fn solve_part1(input: &Map) -> usize {
    num_trees_hit(input, 3, 1)
}

#[aoc(day3, part2)]
fn solve_part2(input: &Map) -> usize {
    let slopes = [
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ];
    slopes.iter()
        .map(|(dc, dr)| num_trees_hit(input, *dc, *dr))
        .fold(1, |acc, x| acc * x)
}