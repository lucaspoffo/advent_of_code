use std::collections::VecDeque;

fn part_a(input: &str) -> u32 {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let mut cost_so_far: Vec<Vec<Option<u32>>> = vec![];
    let mut came_from: Vec<Vec<Option<[usize; 2]>>> = vec![];
    for _ in 0..grid.len() {
        cost_so_far.push(vec![None; grid[0].len()]);
        came_from.push(vec![None; grid[0].len()]);
    }
    cost_so_far[0][0] = Some(0);
    let height = grid.len();
    let width = grid[0].len();
    let mut frontier: VecDeque<[usize; 2]> = VecDeque::new();
    frontier.push_back([0,0]);
    while !frontier.is_empty() {
        let current = frontier.pop_front().unwrap();

        let x = current[0];
        let y = current[1];
        for i in 0..3 {
            for j in 0..3 {
                if (i as i32 - 1).abs() == (j as i32 - 1).abs() { continue; }
                if x == 0 && i == 0 { continue; }
                if y == 0 && j == 0 { continue; }
                if x == width - 1 && i == 2 { continue; }
                if y == height - 1 && j == 2 { continue; }
                
                let next = [x + i - 1, y + j - 1];
                let movement_cost = grid[next[1]][next[0]];
                let new_cost = cost_so_far[y][x].unwrap() + movement_cost;
                if let Some(current_cost) = &mut cost_so_far[next[1]][next[0]] {
                    if new_cost < *current_cost {
                        *current_cost = new_cost;
                        frontier.push_back(next);
                        came_from[next[1]][next[0]] = Some(current);
                    }
                } else {
                    cost_so_far[next[1]][next[0]] = Some(new_cost);
                    frontier.push_back(next);
                    came_from[next[1]][next[0]] = Some(current);
                }
            }
        }
    }
    
    cost_so_far[height - 1][width - 1].unwrap()
}

fn part_b(input: &str) -> u32 {
    let mut grid: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let initial_height = grid.len();
    let initial_width = grid[0].len();
    // Expand grid
    for line in grid.iter_mut() {
        line.append(&mut vec![0u32; 4 * initial_width]);
    }
    for _ in 0..4 * initial_height {
        grid.push(vec![0u32; 5 * initial_width]);
    }

    for i in 0..5 {
        for j in 0..5 {
            if i == 0 && j == 0 { continue; }

            for y in 0..initial_height {
                for x in 0..initial_width {
                    let mut new_value = grid[y][x] + i as u32 + j as u32;
                    if new_value > 9 {
                        new_value -= 9;
                    }
                    grid[j * initial_height + y][i * initial_width + x] = new_value;
                }
            }
        }
    }

    let mut cost_so_far: Vec<Vec<Option<u32>>> = vec![];
    for _ in 0..grid.len() {
        cost_so_far.push(vec![None; grid[0].len()]);
    }
    cost_so_far[0][0] = Some(0);
    let height = grid.len();
    let width = grid[0].len();
    let mut frontier: VecDeque<[usize; 2]> = VecDeque::new();
    frontier.push_back([0,0]);
    while !frontier.is_empty() {
        let current = frontier.pop_front().unwrap();

        let x = current[0];
        let y = current[1];
        for i in 0..3 {
            for j in 0..3 {
                if (i as i32 - 1).abs() == (j as i32 - 1).abs() { continue; }
                if x == 0 && i == 0 { continue; }
                if y == 0 && j == 0 { continue; }
                if x == width - 1 && i == 2 { continue; }
                if y == height - 1 && j == 2 { continue; }
                
                let next = [x + i - 1, y + j - 1];
                let movement_cost = grid[next[1]][next[0]];
                
                let new_cost = cost_so_far[y][x].unwrap() + movement_cost;
                if let Some(current_cost) = &mut cost_so_far[next[1]][next[0]] {
                    if new_cost < *current_cost {
                        *current_cost = new_cost;
                        frontier.push_back(next);
                    }
                } else {
                    cost_so_far[next[1]][next[0]] = Some(new_cost);
                    frontier.push_back(next);
                }
            }
        }
    }
    cost_so_far[height - 1][width - 1].unwrap()
}

fn main() {
    let input = include_str!("input.txt");
    println!("Day 15 part a: {}", part_a(input));
    println!("Day 15 part b: {}", part_b(input));
}

#[test]
fn test() {
    let input = include_str!("test.txt");
    assert_eq!(part_a(input), 40);
    assert_eq!(part_b(input), 315);
}
