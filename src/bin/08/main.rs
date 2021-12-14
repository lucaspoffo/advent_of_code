fn part_a(input: &str) -> u32 {
    let mut count = 0;
    for line in input.lines() {
        let (_, output) = line.split_once(" | ").unwrap();
        for digits in output.split(' ') {
            match digits.len() {
                2 | 3 | 4 | 7 => count += 1,
                _ => {}
            }
        }
    }
    count
}

// Transform digits to an u8 mask
// Easier to check overlaps between digits
// We can use the number of active bits to count
// overlapping segments between digits
fn digit_to_u8(digit: &str) -> u8 {
    let mut mask = 0;
    for c in digit.chars() {
        let t = c as u8 - b'a';
        mask |= 1 << t;
    }
    mask
}

fn part_b(input: &str) -> u32 {
    let mut result = 0;
    for line in input.lines() {
        let (numbers, output) = line.split_once(" | ").unwrap();
        let mut mapping = [0u8; 10];
        let mut two_three_five = Vec::new(); // Len 5
        let mut zero_six_nine = Vec::new(); // Len 6

        for digits in numbers.split(' ') {
            let mask = digit_to_u8(digits);
            match digits.len() {
                2 => mapping[1] = mask,
                3 => mapping[7] = mask,
                4 => mapping[4] = mask,
                7 => mapping[8] = mask,
                5 => two_three_five.push(mask),
                _ => zero_six_nine.push(mask),
            }
        }

        let one = mapping[1];
        let four = mapping[4];

        for value in two_three_five.into_iter() {
            let overlap_one = value & one;
            let overlap_four = value & four;
            if overlap_one.count_ones() == 2 {
                mapping[3] = value;
            } else if overlap_four.count_ones() == 2 {
                mapping[2] = value;
            } else {
                mapping[5] = value;
            }
        }

        for value in zero_six_nine.into_iter() {
            let overlap_one = value & one;
            let overlap_four = value & four;
            if overlap_four.count_ones() == 4 {
                mapping[9] = value;
            } else if overlap_one.count_ones() == 2 {
                mapping[0] = value;
            } else {
                mapping[6] = value;
            }
        }

        let mut value = 0;
        for digits in output.split(' ') {
            let mask = digit_to_u8(digits);
            let position = mapping.iter().position(|x| *x == mask).unwrap();
            value = value * 10 + position as u32;
        }

        result += value;
    }

    result
}

fn main() {
    let input = include_str!("input.txt");
    println!("Day 08 part a: {}", part_a(input));
    println!("Day 08 part b: {}", part_b(input));
}

#[test]
fn test() {
    let input = include_str!("test.txt");
    assert_eq!(part_a(input), 26);
    assert_eq!(part_b(input), 61229);
}
