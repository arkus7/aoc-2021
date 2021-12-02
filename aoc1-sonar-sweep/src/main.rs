use aoc_common::read_input;

fn main() -> Result<(), std::io::Error> {
    let input = read_input("aoc1-sonar-sweep/input.txt")?;
    let measurements = input
        .split("\n")
        .filter_map(|line| line.parse().ok())
        .collect();
    let increases = count_depth_increases(&measurements);
    println!("{}", increases);
    Ok(())
}

fn count_depth_increases(measurements: &Vec<usize>) -> usize {
    measurements
        .windows(2)
        .filter_map(|window| if window[0] < window[1] { Some(()) } else { None } )
        .count()
}

#[cfg(test)]
mod tests {
    use crate::count_depth_increases;

    #[test]
    fn basic_example() {
        let input = vec![199, 200, 201];
        assert_eq!(count_depth_increases(&input), 2);
    }

    #[test]
    fn example_from_website() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(count_depth_increases(&input), 7)
    }
}
