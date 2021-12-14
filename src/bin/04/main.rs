#[derive(Debug)]
struct Board {
    numbers: [u8; 25],
    marked: [bool; 25],
}

impl Board {
    fn from_values(values: &[u8]) -> Self {
        assert_eq!(values.len(), 25);
        let mut numbers = [0u8; 25];
        for (i, value) in values.iter().enumerate() {
            numbers[i] = *value;
        }
        Self {
            numbers,
            marked: [false; 25],
        }
    }

    fn mark(&mut self, number: u8) {
        if let Some(position) = self.numbers.iter().position(|x| *x == number) {
            self.marked[position] = true;
        }
    }

    fn has_won(&self) -> bool {
        for i in 0..5 {
            let mut row = 0;
            let mut col = 0;
            for j in 0..5 {
                if self.marked[i * 5 + j] {
                    row += 1;
                }
                if self.marked[i + j * 5] {
                    col += 1;
                }
            }
            if row == 5 || col == 5 {
                return true;
            }
        }

        false
    }

    fn score(&self, last_number: u8) -> u32 {
        let mut score = 0;
        for (i, marked) in self.marked.iter().enumerate() {
            if !marked {
                score += self.numbers[i] as u32;
            }
        }
        score * (last_number as u32)
    }
}

fn parse_input(input: &str) -> (Vec<u8>, Vec<Board>) {
    let mut lines = input.lines();
    let draw_order: Vec<u8> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let mut numbers = vec![];
    for line in lines {
        if line.is_empty() {
            continue;
        }

        for number in line.split_whitespace() {
            numbers.push(number.parse().unwrap());
        }
    }
    let mut boards: Vec<Board> = Vec::with_capacity(numbers.len() / 25);
    for values in numbers.chunks(25) {
        boards.push(Board::from_values(values));
    }
    (draw_order, boards)
}

fn part_a(input: &str) -> u32 {
    let (draw_order, mut boards) = parse_input(input);

    for number in draw_order.into_iter() {
        for board in boards.iter_mut() {
            board.mark(number);
            if board.has_won() {
                return board.score(number);
            }
        }
    }

    panic!("Winning board not found.");
}

fn part_b(input: &str) -> u32 {
    let (draw_order, mut boards) = parse_input(input);
    let mut score = 0;
    for number in draw_order.into_iter() {
        for board in boards.iter_mut() {
            board.mark(number);
        }
        if boards.len() != 1 {
            boards.retain(|b| !b.has_won());
        } else {
            score = boards[0].score(number);
            break;
        }
    }

    score
}

fn main() {
    let input = include_str!("input.txt");
    println!("Day 04 part a: {}", part_a(input));
    println!("Day 04 part b: {}", part_b(input));
}

#[test]
fn test() {
    let input = include_str!("test.txt");
    assert_eq!(part_a(input), 4512);
    assert_eq!(part_b(input), 1924);
}
