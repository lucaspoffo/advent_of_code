fn score_corrupted(line: &str) -> u32 {
    let mut stack = vec![];
    for c in line.chars() {
        match c {
            '(' | '{' | '<' | '[' => stack.push(c),
            ')' => if stack.pop().unwrap() != '(' { return 3; }
            '}' => if stack.pop().unwrap() != '{' { return 57; }
            ']' => if stack.pop().unwrap() != '[' { return 1197; }
            '>' => if stack.pop().unwrap() != '<' { return 25137; }
            _ => panic!("Invalid char"),
        }
    }

    0
}

fn part_a(input: &str) -> u32 {
    let mut score = 0;
    for line in input.lines() {
        score += score_corrupted(line);
    }

    score
}

fn score_incomplete(line: &str) -> Option<u64> {
    let mut stack = vec![];
    for c in line.chars() {
        match c {
            '(' | '{' | '<' | '[' => stack.push(c),
            ')' => if stack.pop().unwrap() != '(' { return None; }
            '}' => if stack.pop().unwrap() != '{' { return None; }
            ']' => if stack.pop().unwrap() != '[' { return None; }
            '>' => if stack.pop().unwrap() != '<' { return None; }
            _ => panic!("Invalid char"),
        }
    }

    let mut score = 0;
    for c in stack.iter().rev() {
        score *= 5;
        match c {
            '(' => score += 1,
            '[' => score += 2,
            '{' => score += 3,
            '<' => score += 4,
            _ => panic!("Invalid char {}", c),
        }
    }

    Some(score)
}

fn part_b(input: &str) -> u64 {
    let mut scores = vec![];
    for line in input.lines() {
        if let Some(score) = score_incomplete(line) {
            scores.push(score);
        }
    }

    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn main() {
    let input = include_str!("input.txt");
    println!("Day 10 part a: {}", part_a(input));
    println!("Day 10 part b: {}", part_b(input));
}

#[test]
fn test() {
    let input = include_str!("test.txt");
    assert_eq!(part_a(input), 26397);
    assert_eq!(part_b(input), 288957);
}
