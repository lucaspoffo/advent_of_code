fn part_a(mut grid: [[u8; 10]; 10]) -> u32 {
    let mut count = 0;
    for _ in 0..100 {
        for i in 0..10 {
            for j in 0..10 {
                grid[i][j] += 1;
            }
        }

        loop {
            let mut keep_trying = false;
            for i in 0..10 {
                for j in 0..10 {
                    if grid[i][j] == 10 {
                        count += 1;
                        grid[i][j] += 1;
                        for a in 0..3 {
                            for b in 0..3 {
                                if a == 1 && b == 1 { continue; }
                                if i == 0 && a == 0 { continue; }
                                if j == 0 && b == 0 { continue; }
                                if i == 9 && a == 2 { continue; }
                                if j == 9 && b == 2 { continue; }

                                let value = &mut grid[i + a - 1][j + b - 1];
                                if *value < 10 {
                                    *value += 1;
                                    if *value == 10 {
                                        keep_trying = true;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if !keep_trying {
                for i in 0..10 {
                    for j in 0..10 {
                        if grid[i][j] >= 10 {
                            grid[i][j] = 0;
                        }
                    }
                }
                break;
            }
        }
    }

    count
}

fn part_b(mut grid: [[u8; 10]; 10]) -> u32 {
    let mut count = 0;
    'outer: loop {
        count += 1;
        for i in 0..10 {
            for j in 0..10 {
                grid[i][j] += 1;
            }
        }

        loop {
            let mut keep_trying = false;
            for i in 0..10 {
                for j in 0..10 {
                    if grid[i][j] == 10 {
                        grid[i][j] += 1;
                        for a in 0..3 {
                            for b in 0..3 {
                                if a == 1 && b == 1 { continue; }
                                if i == 0 && a == 0 { continue; }
                                if j == 0 && b == 0 { continue; }
                                if i == 9 && a == 2 { continue; }
                                if j == 9 && b == 2 { continue; }

                                let value = &mut grid[i + a - 1][j + b - 1];
                                if *value < 10 {
                                    *value += 1;
                                    if *value == 10 {
                                        keep_trying = true;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if !keep_trying {
                let mut all_flashed = true;
                for i in 0..10 {
                    for j in 0..10 {
                        if grid[i][j] >= 10 {
                            grid[i][j] = 0;
                        } else {
                            all_flashed = false;
                        }
                    }
                }
                if all_flashed {
                    break 'outer;
                }
                break;
            }
        }
    }

    count
}

fn main() {
    let input = [
        [8, 2, 5, 8, 7, 4, 1, 2, 5, 4],
        [3, 3, 3, 5, 2, 8, 6, 2, 1, 1],
        [8, 4, 6, 8, 6, 6, 1, 3, 1, 1],
        [6, 1, 6, 4, 5, 7, 8, 3, 5, 3],
        [2, 1, 3, 8, 4, 1, 4, 5, 5, 3],
        [1, 7, 8, 5, 3, 8, 5, 4, 4, 7],
        [3, 4, 4, 1, 1, 3, 3, 7, 5, 1],
        [3, 5, 8, 6, 8, 6, 2, 8, 3, 7],
        [7, 5, 6, 8, 2, 7, 2, 8, 7, 8],
        [6, 8, 3, 3, 6, 4, 3, 1, 4, 4],
    ];
    println!("Day 11 part a: {}", part_a(input));
    println!("Day 11 part b: {}", part_b(input));
}

#[test]
fn test() {
    let input = [
        [5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
        [2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
        [5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
        [6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
        [6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
        [4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
        [2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
        [6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
        [4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
        [5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
    ];
    assert_eq!(part_a(input), 1656);
    assert_eq!(part_b(input), 195);
}
