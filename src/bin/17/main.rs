use std::ops::RangeInclusive;

// Let's brute force this!
fn part_a(target_x: RangeInclusive<i64>, target_y: RangeInclusive<i64>) -> i64 {
    let mut max_y = 0;
    for i in 0..300 {
        for j in 0..1000i64 {
            let mut velocity = [i, j];
            let mut pos: [i64; 2] = [0, 0];
            let mut run_max_y = 0;
            for _ in 0..1000 {
                run_max_y = run_max_y.max(pos[1]);
                pos[0] += velocity[0];
                pos[1] += velocity[1];
                if velocity[0] > 0 {
                    velocity[0] -= 1;
                }
                velocity[1] -= 1;
                if target_x.contains(&pos[0]) && target_y.contains(&pos[1]) {
                    max_y = max_y.max(run_max_y);
                }
            }
        }
    }
    max_y
}

// Let's also brute force this!
fn part_b(target_x: RangeInclusive<i64>, target_y: RangeInclusive<i64>) -> i64 {
    let mut count = 0;
    for i in 0..300i64 {
        for j in -1000..1000i64 {
            let mut velocity = [i, j];
            let mut pos: [i64; 2] = [0, 0];

            for _ in 0..1000 {
                pos[0] += velocity[0];
                pos[1] += velocity[1];
                if velocity[0] > 0 {
                    velocity[0] -= 1;
                }
                velocity[1] -= 1;
                if target_x.contains(&pos[0]) && target_y.contains(&pos[1]) {
                    count += 1;
                    break;
                }
            }
        }
    }
    
    count
}
fn main() {
    println!("Day 17 part a: {}", part_a(207..=263, -115..=-63));
    println!("Day 17 part b: {}", part_b(207..=263, -115..=-63));
}

#[test]
fn test() {
    assert_eq!(part_a(20..=30, -10..=-5), 45);
    assert_eq!(part_b(20..=30, -10..=-5), 112);
}
