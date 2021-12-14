struct Line {
    from: [u16; 2],
    to: [u16; 2],
}

impl Line {
    fn parse(input: &str) -> Self {
        let (from, to) = input.split_once(" -> ").unwrap();
        let from = from.split_once(',').unwrap();
        let from: [u16; 2] = [from.0.parse().unwrap(), from.1.parse().unwrap()];
        let to = to.split_once(',').unwrap();
        let to: [u16; 2] = [to.0.parse().unwrap(), to.1.parse().unwrap()];

        Self { from, to }
    }

    fn is_horizontal(&self) -> bool {
        self.from[1] == self.to[1]
    }

    fn is_vertical(&self) -> bool {
        self.from[0] == self.to[0]
    }
}

fn part_a(input: &str) -> u32 {
    let mut grid = vec![[0u8; 1000]; 1000];
    for line in input.lines() {
        let line = Line::parse(line);
        if line.is_horizontal() {
            let start = u16::min(line.from[0], line.to[0]);
            let end = u16::max(line.from[0], line.to[0]);
            for i in start..=end {
                grid[line.from[1] as usize][i as usize] += 1;
            }
        } else if line.is_vertical() {
            let start = u16::min(line.from[1], line.to[1]);
            let end = u16::max(line.from[1], line.to[1]);
            for i in start..=end {
                grid[i as usize][line.from[0] as usize] += 1;
            }
        }
    }
    let mut above_two = 0;
    for row in grid.into_iter() {
        for value in row.into_iter() {
            if value >= 2 {
                above_two += 1;
            }
        }
    }
    above_two
}

use std::cmp::Ordering;

fn part_b(input: &str) -> u32 {
    let mut grid = vec![[0u8; 1000]; 1000];
    for line in input.lines() {
        let line = Line::parse(line);
        let mut current = line.from;
        grid[current[1] as usize][current[0] as usize] += 1;
        while line.to != current {
            match current[0].cmp(&line.to[0]) {
                Ordering::Less => current[0] += 1,
                Ordering::Greater => current[0] -= 1,
                Ordering::Equal => {}
            }
            match current[1].cmp(&line.to[1]) {
                Ordering::Less => current[1] += 1,
                Ordering::Greater => current[1] -= 1,
                Ordering::Equal => {}
            }
            grid[current[1] as usize][current[0] as usize] += 1;
        }
    }

    let mut overlaps = 0;
    for row in grid.into_iter() {
        for value in row.into_iter() {
            if value >= 2 {
                overlaps += 1;
            }
        }
    }
    overlaps
}

fn main() {
    let input = include_str!("input.txt");
    println!("Day 05 part a: {}", part_a(input));
    println!("Day 05 part b: {}", part_b(input));
}

#[test]
fn test() {
    let input = include_str!("test.txt");
    assert_eq!(part_a(input), 5);
    assert_eq!(part_b(input), 12);
}
