fn part_a(input: &str) -> u32 {
    let mut depth = 0;
    let mut horizontal_position = 0;
    for line in input.lines() {
        let (direction, amount) = line.split_once(' ').unwrap();
        let amount: u32 = amount.parse().unwrap();
        match direction {
            "forward" => horizontal_position += amount,
            "up" => depth -= amount,
            "down" => depth += amount,
            _ => panic!("Invalid direction {}", direction)
        }
    }

    depth * horizontal_position
}

fn part_b(input: &str) -> u32 {
    let mut depth = 0;
    let mut horizontal_position = 0;
    let mut aim = 0;
    for line in input.lines() {
        let (direction, amount) = line.split_once(' ').unwrap();
        let amount: u32 = amount.parse().unwrap();
        match direction {
            "forward" => {
                horizontal_position += amount;
                depth += aim * amount;
            }
            "up" => aim -= amount,
            "down" => aim += amount,
            _ => panic!("Invalid direction {}", direction)
        }
    }
    depth * horizontal_position
}

fn main() {
    let input = include_str!("input.txt");
    println!("Day 02 part a: {}", part_a(input));
    println!("Day 02 part b: {}", part_b(input));
}

#[test]
fn test() {
    let input = include_str!("test.txt");
    assert_eq!(part_a(input), 150);
    assert_eq!(part_b(input), 900);
}
