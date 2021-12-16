use std::collections::HashMap;

fn is_small_cave(cave: &str) -> bool {
    let first_char = cave.as_bytes()[0];
    first_char > b'a' && first_char < b'z' && cave != "start" && cave != "end"
}

fn parse_connections(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let (from, to) = line.split_once('-').unwrap();
        if to != "start" && from != "end" {
            let paths = map.entry(from).or_insert_with(Vec::new);
            paths.push(to);
        }
        if from != "start" && to != "end" {
            let paths = map.entry(to).or_insert_with(Vec::new);
            paths.push(from);
        }
    }
    map
}

fn part_a(input: &str) -> usize {
    let map = parse_connections(input);
    // Keeping track of paths is not needed for the solution but is good for debugging
    let mut paths: Vec<Vec<&str>> = vec![];
    let mut stack = vec![vec!["start"]];
    let mut current_path: Vec<&str> = vec![];
    loop {
        let mut remove_last = false;
        if let Some(current_available_caves) = stack.last_mut() {
            if let Some(current) = current_available_caves.pop() {
                current_path.push(current);
                if current == "end" {
                    paths.push(current_path.clone());
                    current_path.pop();
                    continue;
                }
                let mut available_caves = map.get(current).unwrap().clone();
                available_caves.retain(|c| {
                    if is_small_cave(c) {
                        return !current_path.contains(c);
                    }
                    true
                });
                if available_caves.is_empty() {
                    current_path.pop();
                } else {
                    stack.push(available_caves);
                }
            } else {
                remove_last = true;
            }
        } else {
            break;
        }
        if remove_last {
            stack.pop();
            current_path.pop();
        }
    }

    paths.len()
}

#[derive(Debug, Default, Clone)]
struct Path<'a> {
    caves: Vec<&'a str>,
    small_cave_twice: Option<&'a str>,
}

impl<'a> Path<'a> {
    fn pop(&mut self) {
        if let (Some(last), Some(twice)) = (self.caves.pop(), self.small_cave_twice) {
            if last == twice {
                self.small_cave_twice = None;
            }
        }
    }

    fn push(&mut self, cave: &'a str) {
        if is_small_cave(cave) && self.caves.contains(&cave) {
            assert!(self.small_cave_twice.is_none());
            self.small_cave_twice = Some(cave);
        }
        self.caves.push(cave);
    }

    fn path_available(&mut self, cave: &str) -> bool {
        if self.small_cave_twice.is_some() && is_small_cave(cave) {
            return !self.caves.contains(&cave);
        }
        true
    }
}

fn part_b(input: &str) -> usize {
    let map = parse_connections(input);
    // Keeping track of paths is not needed for the solution but is good for debugging
    let mut paths: Vec<Path> = vec![];
    let mut stack = vec![vec!["start"]];
    let mut current_path = Path::default();

    loop {
        let mut remove_last = false;
        if let Some(current_available_caves) = stack.last_mut() {
            if let Some(current) = current_available_caves.pop() {
                current_path.push(current);
                if current == "end" {
                    paths.push(current_path.clone());
                    current_path.pop();
                    continue;
                }
                let mut available_caves = map.get(current).unwrap().clone();
                available_caves.retain(|c| current_path.path_available(c));
                if available_caves.is_empty() {
                    current_path.pop();
                } else {
                    stack.push(available_caves);
                }
            } else {
                remove_last = true;
            }
        } else {
            break;
        }
        if remove_last {
            stack.pop();
            current_path.pop();
        }
    }

    paths.len()
}

#[derive(Debug)]
struct MapA<'a> {
    connections: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> MapA<'a> {
    fn parse(input: &'a str) -> Self {
        let mut connections: HashMap<&str, Vec<&str>> = HashMap::new();
        for line in input.lines() {
            let (from, to) = line.split_once('-').unwrap();
            if to != "start" && from != "end" {
                let paths = connections.entry(from).or_insert_with(Vec::new);
                paths.push(to);
            }
            if from != "start" && to != "end" {
                let paths = connections.entry(to).or_insert_with(Vec::new);
                paths.push(from);
            }
        }

        Self { connections }
    }

    fn visit(&self, cave: &'a str, count: &mut usize, small_caves_visited: &mut Vec<&'a str>) {
        if cave == "end" {
            *count += 1;
            return;
        }
        if is_small_cave(cave) {
            small_caves_visited.push(cave);
        }

        let available_caves = self.connections.get(cave).unwrap();
        for path in available_caves.iter() {
            if is_small_cave(path) && small_caves_visited.contains(path) {
                continue;
            }
            self.visit(path, count, small_caves_visited);
        }

        small_caves_visited.retain(|c| c != &cave);
    }

    fn count_paths(&self) -> usize {
        let mut count = 0;
        let mut small_caves_visited = vec![];
        self.visit("start", &mut count, &mut small_caves_visited);

        count
    }
}

fn part_a_2(input: &str) -> usize {
    let map = MapA::parse(input);
    map.count_paths()
}

#[derive(Debug)]
struct MapB<'a> {
    connections: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> MapB<'a> {
    fn parse(input: &'a str) -> Self {
        let mut connections: HashMap<&str, Vec<&str>> = HashMap::new();
        for line in input.lines() {
            let (from, to) = line.split_once('-').unwrap();
            if to != "start" && from != "end" {
                let paths = connections.entry(from).or_insert_with(Vec::new);
                paths.push(to);
            }
            if from != "start" && to != "end" {
                let paths = connections.entry(to).or_insert_with(Vec::new);
                paths.push(from);
            }
        }

        Self { connections }
    }

    fn visit(&self, cave: &'a str, count: &mut usize, small_caves_visited: &mut Vec<&'a str>, small_cave_twice: &mut Option<&'a str>) {
        if cave == "end" {
            *count += 1;
            return;
        }

        if is_small_cave(cave) {
            if small_caves_visited.contains(&cave) {
                assert!(small_cave_twice.is_none());
                *small_cave_twice = Some(cave);
            } else {
                small_caves_visited.push(cave);
            }
        }

        let available_caves = self.connections.get(cave).unwrap();
        for path in available_caves.iter() {
            if small_cave_twice.is_some() && is_small_cave(path) && small_caves_visited.contains(path) {
                continue;
            }
            self.visit(path, count, small_caves_visited, small_cave_twice);
        }

        if let Some(twice) = small_cave_twice {
            if twice == &cave {
                *small_cave_twice = None;
                return;
            }
        }

        small_caves_visited.retain(|c| c != &cave);
    }

    fn count_paths(&self) -> usize {
        let mut count = 0;
        let mut small_caves_visited = vec![];
        let mut small_cave_twice = None;
        self.visit("start", &mut count, &mut small_caves_visited, &mut small_cave_twice);

        count
    }
}

fn part_b_2(input: &str) -> usize {
    let map = MapB::parse(input);
    map.count_paths()
}

fn main() {
    let input = include_str!("input.txt");
    println!("Day 12 part a: {}", part_a(input));
    println!("Day 12 part b: {}", part_b(input));
    println!("Day 12 part a2: {}", part_a_2(input));
    println!("Day 12 part b2: {}", part_b_2(input));
}

#[test]
fn test() {
    let input = include_str!("test.txt");
    assert_eq!(part_a(input), 10);
    assert_eq!(part_a_2(input), 10);
    assert_eq!(part_b(input), 36);
    assert_eq!(part_b_2(input), 36);
}
