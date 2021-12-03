use std::collections::HashMap;

use aoc_common::read_input;

fn main() -> std::io::Result<()> {
    let input = read_input("aoc3-binary-diagnostic/input.txt")?;

    let lines = input
        .split("\n")
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>();

    solve_part_one(&lines);

    Ok(())
}

fn solve_part_one(lines: &Vec<&str>) {
    let number_of_bits = lines.first().unwrap().len();
    let mut gamma_rate_str = String::with_capacity(number_of_bits);

    for i in 0..=number_of_bits - 1 {
        let mut zeros = 0;
        let mut ones = 0;

        for &line in lines.iter() {
            match line.chars().nth(i).unwrap() {
                '0' => zeros += 1,
                '1' => ones += 1,
                _ => {}
            }
        }

        let most_common = {
            if zeros > ones {
                '0'
            } else {
                '1'
            }
        };

        gamma_rate_str.push(most_common);
    }

    let epsilon_rate_str = gamma_rate_str
        .chars()
        .map(|c| if c == '0' { '1' } else { '0' })
        .collect::<String>();
    let gamma_rate = usize::from_str_radix(&gamma_rate_str, 2).unwrap();
    let epsilon_rate = usize::from_str_radix(&epsilon_rate_str, 2).unwrap();

    println!(
        "Gamma: {}, Epsilon: {}, Answer: {}",
        gamma_rate,
        epsilon_rate,
        gamma_rate * epsilon_rate
    );
}
