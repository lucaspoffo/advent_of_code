fn part_a(input: &str) -> u32 {
    let mut ones = [0u32; 16];
    let mut zeros = [0u32; 16];
    let mut length = 0;
    for line in input.lines() {
        length = line.len();
        for (i, binary) in line.chars().enumerate() {
            match binary {
                '0' => zeros[i] += 1,
                '1' => ones[i] += 1,
                _ => panic!("Invalid binary {}", binary),
            }
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;
    for i in 0..length {
        let mask = 1 << (length - i - 1);
        if ones[i] > zeros[i] {
            gamma |= mask;
        } else {
            epsilon |= mask;
        }
    }

    gamma * epsilon
}

#[derive(Debug)]
enum Rating {
    CO2Scrubber,
    OxygenGenerator,
}

fn part_b(input: &str) -> u32 {
    let numbers: Vec<&str> = input.lines().collect();
    let oxygen_generator = calculate_rating(Rating::OxygenGenerator, &numbers);
    let co2_scrubber = calculate_rating(Rating::CO2Scrubber, &numbers);

    oxygen_generator * co2_scrubber
}

fn calculate_rating(rating: Rating, numbers: &[&str]) -> u32 {
    let mut skipped = vec![false; numbers.len()];
    let mut remaing = numbers.len();
    let mut current_position = 0;
    while remaing > 1 {
        let criteria = bit_criteria(&rating, current_position, numbers, &skipped);
        for (i, number) in numbers.iter().enumerate() {
            let c = number.chars().nth(current_position).unwrap();
            if !skipped[i] && c != criteria {
                skipped[i] = true;
                remaing -= 1;
            }
        }
        current_position += 1;
    }

    let position = skipped.iter().position(|s| !s).unwrap();
    u32::from_str_radix(numbers[position], 2).unwrap()
}

fn bit_criteria(rating: &Rating, position: usize, numbers: &[&str], skipped: &[bool]) -> char {
    let mut zeros = 0;
    let mut ones = 0;
    for (i, number) in numbers.iter().enumerate() {
        if skipped[i] {
            continue;
        }
        match number.chars().nth(position) {
            Some('0') => zeros += 1,
            Some('1') => ones += 1,
            _ => unreachable!(),
        }
    }

    match rating {
        Rating::CO2Scrubber => if zeros <= ones { '0' } else { '1' }
        Rating::OxygenGenerator => if ones >= zeros { '1' } else { '0' }
    }
}

fn main() {
    let input = include_str!("input.txt");
    println!("Day 03 part a: {}", part_a(input));
    println!("Day 03 part b: {}", part_b(input));
}

#[test]
fn test() {
    let input = include_str!("test.txt");
    assert_eq!(part_a(input), 198);
    assert_eq!(part_b(input), 230);
}
