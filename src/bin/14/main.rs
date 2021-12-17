use std::collections::HashMap;

fn part_a(input: &str, steps: usize) -> usize {
    let mut lines = input.lines();
    let mut template: String = lines.next().unwrap().to_string();
    let mut rules: HashMap<&str, char> = HashMap::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let (rule, result) = line.split_once(" -> ").unwrap();
        let result = result.chars().next().unwrap();
        rules.insert(rule, result);
    }

    for _ in 0..steps {
        let mut results: Vec<(usize, char)> = vec![];
        for i in 0..template.len() - 1 {
            let window = &template[i..i + 2];
            for (rule, result) in rules.iter() {
                if *rule == window {
                    results.push((i, *result));
                    break;
                }
            }
        }
        results.sort_by(|a, b| a.0.cmp(&b.0));
        for (i, (pos, c)) in results.iter().enumerate() {
            template.insert(pos + i + 1, *c);
        }
    }

    // Count
    let mut char_count: HashMap<char, usize> = HashMap::new();
    for c in template.chars() {
        let count = char_count.entry(c).or_insert(0);
        *count += 1;
    }

    let max = char_count.values().max().unwrap();
    let min = char_count.values().min().unwrap();

    max - min
}

// Solution used in part a is too slow for 40 iterations
fn part_b(input: &str, steps: usize) -> usize {
    let mut lines = input.lines();
    let template: String = lines.next().unwrap().to_string();
    let mut rules: HashMap<[char; 2], char> = HashMap::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let (rule, result) = line.split_once(" -> ").unwrap();
        let rule: Vec<char> = rule.chars().collect();
        let result = result.chars().next().unwrap();
        rules.insert([rule[0], rule[1]], result);
    }
    // Instead of keeping tracking of the whole template string
    // We keep track of the pairs count in the template
    let mut pairs: HashMap<[char; 2], usize> = HashMap::new();
    for i in 0..template.len() - 1 {
        let pair: Vec<char> = template[i..i + 2].chars().collect();
        let pair = [pair[0], pair[1]];
        let count = pairs.entry(pair).or_insert(0);
        *count += 1;
    }

    for _ in 0..steps {
        for (pair, count) in pairs.clone().iter() {
            if *count == 0 {
                continue;
            }

            if let Some(insert) = rules.get(pair) {
                let c = pairs.get_mut(pair).unwrap();
                // The pairs is split, into 2 new pairs, ex:
                // rule:      CH -> B
                // new pairs: CH -> CB BH
                // So we need to remove the count from the split one
                *c -= count;
                // And add the count for the new pairs
                let first = [pair[0], *insert];
                let c = pairs.entry(first).or_insert(0);
                *c += count;
                let second = [*insert, pair[1]];
                let c = pairs.entry(second).or_insert(0);
                *c += count;
            }
        }
    }

    // Count pairs
    // Use the first char from each pair to count
    let mut char_count: HashMap<char, usize> = HashMap::new();
    for (pair, count) in pairs.iter() {
        let c = char_count.entry(pair[0]).or_insert(0);
        *c += count;
    }

    // The last char is a special case, since it never leaves the last position
    let last_char = template.chars().last().unwrap();
    let last_count = char_count.entry(last_char).or_insert(0);
    *last_count += 1;

    let max = char_count.values().max().unwrap();
    let min = char_count.values().min().unwrap();

    max - min
}

fn main() {
    let input = include_str!("input.txt");
    println!("Day 14 part a: {}", part_a(input, 10));
    println!("Day 14 part b: {}", part_b(input, 40));
}

#[test]
fn test() {
    let input = include_str!("test.txt");
    assert_eq!(part_a(input, 10), 1588);
    assert_eq!(part_b(input, 10), 1588);
    assert_eq!(part_b(input, 40), 2188189693529);
}
