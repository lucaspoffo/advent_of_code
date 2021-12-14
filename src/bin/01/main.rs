fn part_a(input: &str) -> u32 {
    let mut lines = input.lines();
    let mut last: u32 = lines.next().unwrap().parse().unwrap();
    let mut count = 0;
    for line in lines {
        let depth: u32 = line.parse().unwrap();
        if depth > last {
            count += 1;
        }
        last = depth;
    }
    count
}

fn part_b(input: &str) -> u32 {
    let mut lines = input.lines();
    let mut first: u32 = lines.next().unwrap().parse().unwrap();
    let mut second: u32 = lines.next().unwrap().parse().unwrap();
    let mut last = u32::MAX;
    let mut count = 0;
    for line in lines {
        let third: u32 = line.parse().unwrap();
        let sum = first + second + third;
        if sum > last {
            count += 1;
        }
        last = sum;
        first = second;
        second = third;
    }

    count
}

fn main() {
    let input = include_str!("input.txt");
    println!("Day 01 part a: {}", part_a(input));
    println!("Day 01 part b: {}", part_b(input));
}

#[test]
fn test() {
    let input = include_str!("test.txt");
    assert_eq!(part_a(input), 7);
    assert_eq!(part_b(input), 5);
}
