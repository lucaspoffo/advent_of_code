fn part_a(input: &str, days: u64) -> u64 {
    let mut anglefishs = [0u64; 9];
    for number in input.trim_end().split(',') {
        let index: usize = number.parse().unwrap();
        anglefishs[index] += 1;
    }
    
    for _ in 0..days {
        anglefishs.rotate_left(1);
        // Add old fishs to 6 lifespan
        anglefishs[6] += anglefishs[8];
    }

    anglefishs.iter().sum()
}

fn main() {
    let input = include_str!("input.txt");
    println!("Day 06 part a: {}", part_a(input, 80));
    println!("Day 06 part b: {}", part_a(input, 256));
}

#[test]
fn test() {
    let input = include_str!("test.txt");
    assert_eq!(part_a(input, 80), 5934);
    assert_eq!(part_a(input, 256), 26984457539);
}
