use std::{ops::Deref, str::FromStr};

use aoc_common::read_input;

#[derive(PartialEq, Debug)]
enum Command {
    Forward(usize),
    Down(usize),
    Up(usize),
}

#[derive(Debug, Clone, PartialEq)]
enum CommandParseError {
    InvalidCommandName,
    InvalidValue,
    InvalidFormat,
}

#[derive(Debug, Clone, PartialEq)]
struct Position {
    horizontal: usize,
    depth: usize,
    aim: usize,
}

impl FromStr for Command {
    type Err = CommandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err(CommandParseError::InvalidFormat);
        }
        let command = parts.get(0).unwrap().deref();
        let value: usize = parts
            .get(1)
            .unwrap()
            .parse()
            .map_err(|_| CommandParseError::InvalidValue)?;
        match command {
            "forward" => Ok(Self::Forward(value)),
            "down" => Ok(Self::Down(value)),
            "up" => Ok(Self::Up(value)),
            _ => Err(CommandParseError::InvalidCommandName),
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let input = read_input("aoc2-dive/input.txt")?
        .split('\n')
        .filter_map(|line| line.parse::<Command>().ok())
        .collect::<Vec<_>>();

    let pos1 = calculate_position(&input);
    println!("Part 1: {}", pos1.horizontal * pos1.depth);

    let pos2 = calculate_position_aim(&input);
    println!("Part 2: {}", pos2.horizontal * pos2.depth);
    Ok(())
}

fn calculate_position(commands: &[Command]) -> Position {
    let mut pos = Position {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };

    for command in commands {
        match command {
            Command::Forward(val) => pos.horizontal += val,
            Command::Down(val) => pos.depth += val,
            Command::Up(val) => pos.depth -= val,
        }
    }

    pos
}

fn calculate_position_aim(commands: &[Command]) -> Position {
    let mut pos = Position {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };

    for command in commands {
        match command {
            Command::Forward(val) => {
                pos.horizontal += val;
                pos.depth += pos.aim * val;
            }
            Command::Down(val) => pos.aim += val,
            Command::Up(val) => pos.aim -= val,
        }
    }

    pos
}

#[cfg(test)]
mod command_tests {
    use crate::{Command, CommandParseError};

    #[test]
    fn parse_forward_cmd() {
        let str = "forward 90";
        let cmd: Command = str.parse().unwrap();
        assert_eq!(cmd, Command::Forward(90));
    }

    #[test]
    fn parse_down_cmd() {
        let str = "down 90";
        let cmd: Command = str.parse().unwrap();
        assert_eq!(cmd, Command::Down(90));
    }

    #[test]
    fn parse_up_cmd() {
        let str = "up 90";
        let cmd: Command = str.parse().unwrap();
        assert_eq!(cmd, Command::Up(90));
    }

    #[test]
    fn parse_invalid_cmd_name() {
        let str = "asd 90";
        let cmd: Result<Command, CommandParseError> = str.parse();
        assert_eq!(cmd.err(), Some(CommandParseError::InvalidCommandName));
    }

    #[test]
    fn parse_invalid_cmd_value() {
        let str = "forward down";
        let cmd: Result<Command, CommandParseError> = str.parse();
        assert_eq!(cmd.err(), Some(CommandParseError::InvalidValue));
    }

    #[test]
    fn parse_invalid_cmd_format() {
        let str = "asd";
        let cmd: Result<Command, CommandParseError> = str.parse();
        assert_eq!(cmd.err(), Some(CommandParseError::InvalidFormat));
    }
}

#[cfg(test)]
mod calculation_tests {
    use crate::{calculate_position, Command, Position};

    #[test]
    fn empty_vec() {
        let pos = calculate_position(&vec![]);
        assert_eq!(
            pos,
            Position {
                horizontal: 0,
                depth: 0,
                aim: 0
            }
        );
    }

    #[test]
    fn move_forward() {
        let pos = calculate_position(&vec![Command::Forward(20)]);
        assert_eq!(
            pos,
            Position {
                horizontal: 20,
                depth: 0,
                aim: 0
            }
        )
    }

    #[test]
    fn move_down() {
        let pos = calculate_position(&vec![Command::Down(20)]);
        assert_eq!(
            pos,
            Position {
                horizontal: 0,
                depth: 20,
                aim: 0
            }
        )
    }

    #[test]
    fn move_down_and_up() {
        let pos = calculate_position(&vec![Command::Down(100), Command::Up(20)]);
        assert_eq!(
            pos,
            Position {
                horizontal: 0,
                depth: 80,
                aim: 0
            }
        )
    }
}

#[cfg(test)]
mod calculation_with_aim_tests {
    use crate::{calculate_position_aim, Command, Position};

    #[test]
    fn empty_vec() {
        let pos = calculate_position_aim(&vec![]);
        assert_eq!(
            pos,
            Position {
                horizontal: 0,
                depth: 0,
                aim: 0
            }
        );
    }

    #[test]
    fn move_forward() {
        let pos = calculate_position_aim(&vec![Command::Forward(20)]);
        assert_eq!(
            pos,
            Position {
                horizontal: 20,
                depth: 0,
                aim: 0
            }
        )
    }

    #[test]
    fn move_down() {
        let pos = calculate_position_aim(&vec![Command::Down(20)]);
        assert_eq!(
            pos,
            Position {
                horizontal: 0,
                depth: 0,
                aim: 20
            }
        )
    }

    #[test]
    fn move_down_and_up() {
        let pos = calculate_position_aim(&vec![Command::Down(100), Command::Up(20)]);
        assert_eq!(
            pos,
            Position {
                horizontal: 0,
                depth: 0,
                aim: 80
            }
        )
    }

    #[test]
    fn move_down_and_forward() {
        let pos = calculate_position_aim(&vec![Command::Down(100), Command::Forward(20)]);
        assert_eq!(
            pos,
            Position {
                horizontal: 20,
                depth: 20 * 100,
                aim: 100,
            }
        )
    }
}
