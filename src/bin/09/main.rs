#[derive(Debug)]
struct HeightMap<'a> {
    values: &'a [u8],
    width: usize,
    height: usize,
}

impl HeightMap<'static> {
    fn parse(input: &'static [u8]) -> Self {
        let width = input.iter().position(|x| *x == b'\n').unwrap();
        let mut height = 0;
        for byte in input.iter() {
            if *byte == b'\n' {
                height += 1
            }
        }
        Self {
            values: input,
            width,
            height,
        }
    }

    fn get(&self, row: usize, col: usize) -> u8 {
        let i = ((self.width + 1) * row) + col;
        self.values[i] - b'0'
    }

    fn risk_level(&self, row: usize, col: usize) -> u8 {
        let depth = self.get(row, col);

        if row > 0 && depth >= self.get(row - 1, col) {
            return 0;
        };
        if row + 1 < self.height && depth >= self.get(row + 1, col) {
            return 0;
        };
        if col > 0 && depth >= self.get(row, col - 1) {
            return 0;
        };
        if col + 1 < self.width && depth >= self.get(row, col + 1) {
            return 0;
        };

        depth + 1
    }

    fn risk_total(&self) -> u32 {
        let mut total = 0;
        for col in 0..self.width {
            for row in 0..self.height {
                total += self.risk_level(row, col) as u32;
            }
        }

        total
    }
}

fn part_a(input: &'static [u8]) -> u32 {
    let height_map = HeightMap::parse(input);
    height_map.risk_total()
}

// Part B
impl HeightMap<'static> {
    fn is_low_point(&self, row: usize, col: usize) -> bool {
        let depth = self.get(row, col);

        if row > 0 && depth >= self.get(row - 1, col) {
            return false;
        };
        if row + 1 < self.height && depth >= self.get(row + 1, col) {
            return false;
        };
        if col > 0 && depth >= self.get(row, col - 1) {
            return false;
        };
        if col + 1 < self.width && depth >= self.get(row, col + 1) {
            return false;
        };

        true
    }

    fn expand_basin(&self, row: usize, col: usize, basin: &mut [bool]) {
        let depth = self.get(row, col);
        if depth == 9 {
            return;
        }

        basin[self.width * row + col] = true;

        if row > 0 && self.get(row - 1, col) > depth {
            self.expand_basin(row - 1, col, basin)
        };
        if row + 1 < self.height && self.get(row + 1, col) > depth {
            self.expand_basin(row + 1, col, basin)
        };
        if col > 0 && self.get(row, col - 1) > depth {
            self.expand_basin(row, col - 1, basin)
        };
        if col + 1 < self.width && self.get(row, col + 1) > depth {
            self.expand_basin(row, col + 1, basin)
        };
    }

    fn find_three_largest_basins_product(&self) -> u32 {
        let mut basin = vec![false; self.width * self.height];
        let mut largest_basins = [0u32; 3];
        for row in 0..self.height {
            for col in 0..self.width {
                if self.is_low_point(row, col) {
                    self.expand_basin(row, col, &mut basin);
                    let mut size = 0;
                    for in_basin in basin.iter() {
                        if *in_basin {
                            size += 1
                        };
                    }
                    if size > largest_basins[0] {
                        largest_basins[0] = size;
                        largest_basins.sort_unstable();
                    }
                    for b in basin.iter_mut() {
                        *b = false
                    }
                }
            }
        }

        largest_basins[0] * largest_basins[1] * largest_basins[2]
    }
}

fn part_b(input: &'static [u8]) -> u32 {
    let height_map = HeightMap::parse(input);
    height_map.find_three_largest_basins_product()
}

fn main() {
    let input = include_bytes!("input.txt");
    println!("Day 09 part a: {}", part_a(input));
    println!("Day 09 part b: {}", part_b(input));
}

#[test]
fn test() {
    let input = include_bytes!("test.txt");
    assert_eq!(part_a(input), 15);
    assert_eq!(part_b(input), 1134);
}
