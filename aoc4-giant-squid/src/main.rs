use aoc_common::read_input;

#[derive(Debug, Clone)]
struct Board {
    items: Vec<BoardItem>,
}

impl Board {
    fn has_won(&self) -> bool {
        for row in 0..5 {
            let row_win = self
                .items
                .iter()
                .filter(|i| i.row == row)
                .all(|item| item.marked);
            if row_win {
                return true;
            }
        }
        for column in 0..5 {
            let column_win = self
                .items
                .iter()
                .filter(|i| i.column == column)
                .all(|i| i.marked);
            if column_win {
                return true;
            }
        }
        false
    }

    fn mark_item(&mut self, value: usize) {
        self.items
            .iter_mut()
            .filter(|i| i.value == value)
            .for_each(|i| i.marked = true);
    }

    fn calculate_score(&self, last_marked: usize) -> usize {
        self.items
            .iter()
            .filter_map(|i| match i.marked {
                true => None,
                false => Some(i.value),
            })
            .sum::<usize>()
            * last_marked
    }
}

#[derive(Debug, Clone)]
struct BoardItem {
    value: usize,
    row: usize,
    column: usize,
    marked: bool,
}

fn main() -> Result<(), std::io::Error> {
    let input = read_input("aoc4-giant-squid/input.txt")?;

    let lines = input.lines();
    let draw_queue = lines
        .clone()
        .take(1)
        .flat_map(|line| line.split(',').collect::<Vec<&str>>())
        .map(|num_str| num_str.parse().unwrap())
        .collect::<Vec<usize>>();

    let boards = &mut lines
        .skip(2)
        .collect::<Vec<&str>>()
        .chunks(6)
        .map(|chunk| {
            println!("Parsing chunk: {:?}", chunk);
            let mut row = 0;
            let mut items = vec![];
            for &line in chunk {
                if line.is_empty() {
                    continue;
                }
                let numbers: Vec<usize> = line
                    .split_whitespace()
                    .filter_map(|str| str.parse().ok())
                    .collect();
                for (idx, num) in numbers.iter().enumerate() {
                    let item = BoardItem {
                        row,
                        column: idx,
                        value: *num,
                        marked: false,
                    };
                    println!("Parsed item: {:?}", item);
                    items.push(item);
                }
                row += 1;
            }
            Board { items }
        })
        .collect::<Vec<Board>>();

    let mut boards2 = boards.clone();

    // Part 1
    for number in draw_queue.clone() {
        boards.iter_mut().for_each(|b| b.mark_item(number));
        let board_won = boards.iter().find(|b| b.has_won());
        if let Some(board) = board_won {
            let score = board.calculate_score(number);
            println!("Score: {:?}", score);
            break;
        }
    }

    // Part 2

    for number in draw_queue {
        boards2.iter_mut().for_each(|b| b.mark_item(number));
        if boards2.len() == 1 {
            let board = boards2.first().unwrap();
            if board.has_won() {
                println!("Score (2): {:?}", board.calculate_score(number));
                break;
            }
        } else {
            boards2 = boards2.into_iter().filter(|b| !b.has_won()).collect();
        }
    }
    Ok(())
}
