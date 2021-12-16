#[derive(Debug)]
pub enum Fold {
    Horizontal(u32),
    Vertical(u32),
}

fn part_a(input: &str) -> usize {
    let mut points = vec![];
    let mut folds = vec![];
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        if line.contains("fold") {
            let (_, value) = line.split_once('=').unwrap();
            let value: u32 = value.parse().unwrap();
            if line.contains('x') {
                folds.push(Fold::Horizontal(value));
            } else {
                folds.push(Fold::Vertical(value));
            }
            continue;
        }

        let (x, y) = line.split_once(",").unwrap();
        let point: [u32; 2] = [x.parse().unwrap(), y.parse().unwrap()];
        points.push(point);
    }

    for fold in folds.iter() {
        for point in points.iter_mut() {
            match fold {
                Fold::Horizontal(at) if point[0] > *at => point[0] = (2 * at) - point[0],
                Fold::Vertical(at) if point[1] > *at => point[1] = (2 * at) - point[1],
                _ => {}
            }
        }
    }
    points.sort_unstable();
    points.dedup();

    for j in 0..7 {
        for i in 0..50 {
            let c = if points.contains(&[i, j]) { '█' } else { ' ' };
            print!("{}", c);
        }
        println!();
    }

    points.len()
}

fn main() {
    let input = include_str!("input.txt");
    println!("Day 13 part a: {}", part_a(input));
}

#[test]
fn test() {
    let input = include_str!("test.txt");
    assert_eq!(part_a(input), 16);
}
