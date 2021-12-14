fn part_a(input: &str) -> u32 {
    let numbers: Vec<u32> = input.trim_end().split(',').map(|x| x.parse().unwrap()).collect();

    let max_value = *numbers.iter().max().unwrap() as usize;
    let mut min_cost = u32::MAX;
    for i in 0..max_value {
        let i = i as u32;
        let mut cost = 0;
        for n in numbers.iter() {
            let max = n.max(&i);
            let min = n.min(&i);
            cost += max - min;
        }
        min_cost = min_cost.min(cost);
    }

    min_cost
}

fn part_b(input: &str) -> u32 {
    let numbers: Vec<u32> = input.trim_end().split(',').map(|x| x.parse().unwrap()).collect();
    let max_value = *numbers.iter().max().unwrap() as usize;

    let mut min_cost = u32::MAX;
    for i in 0..max_value {
        let i = i as u32;
        let mut cost = 0;
        for n in numbers.iter() {
            let max = n.max(&i);
            let min = n.min(&i);
            let distance = max - min;
            cost += distance * (distance + 1) / 2;
        }
        min_cost = min_cost.min(cost);
    }

    min_cost
}

fn main() {
    let input = include_str!("input.txt");
    println!("Day 07 part a: {}", part_a(input));
    println!("Day 07 part b: {}", part_b(input));
}

#[test]
fn test() {
    let input = include_str!("test.txt");
    assert_eq!(part_a(input), 37);
    assert_eq!(part_b(input), 168);
}
