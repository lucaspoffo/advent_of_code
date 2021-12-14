#!/bin/bash

mkdir src/bin/"$1"

cat > src/bin/"$1"/main.rs << EOF
fn part_a(input: &str) -> u32 {
  0
}

fn part_b(input: &str) -> u32 {
  0
}

fn main() {
    let input = include_str!("input.txt");
    println!("Day $1 part a: {}", part_a(input));
    println!("Day $1 part b: {}", part_b(input));
}

#[test]
fn test() {
    let input = include_str!("test.txt");
    assert_eq!(part_a(input), 0);
    assert_eq!(part_b(input), 0);
}
EOF

touch src/bin/"$1"/input.txt
touch src/bin/"$1"/test.txt
