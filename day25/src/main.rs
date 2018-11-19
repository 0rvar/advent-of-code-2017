use regex::{Captures, Regex};
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct State {
    id: char,
    when_zero: Action,
    when_one: Action,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Action {
    write: usize,
    go_direction: Direction,
    next_state: char,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Direction {
    Left,
    Right,
}

static INSTRUCTION_REGEX: &'static str = r"
Begin in state ([A-Z]+)\.
Perform a diagnostic checksum after (\d+) steps\.
";
static STATE_REGEX: &'static str = r"
In state ([A-Z]):
  If the current value is 0:
    - Write the value (\d).
    - Move one slot to the (left|right).
    - Continue with state ([A-Z]+).
  If the current value is 1:
    - Write the value (\d).
    - Move one slot to the (left|right).
    - Continue with state ([A-Z]+).
";

fn main() {
    let input = include_str!("input.txt");

    let instruction_regex = Regex::new(INSTRUCTION_REGEX.trim()).unwrap();
    let instruction_captures = instruction_regex.captures(input).unwrap();
    let initial_state = get_char_match(&instruction_captures, 1);
    let steps = get_number_match(&instruction_captures, 2);
    let states_regex = Regex::new(STATE_REGEX.trim()).unwrap();
    let states = states_regex
        .captures_iter(input)
        .map(|capture| {
            let state_id = get_char_match(&capture, 1);
            let zero_action = Action {
                write: get_number_match(&capture, 2),
                go_direction: get_direction_match(&capture, 3),
                next_state: get_char_match(&capture, 4),
            };
            let one_action = Action {
                write: get_number_match(&capture, 5),
                go_direction: get_direction_match(&capture, 6),
                next_state: get_char_match(&capture, 7),
            };
            State {
                id: state_id,
                when_zero: zero_action,
                when_one: one_action,
            }
        })
        .collect::<Vec<_>>();

    let mut tape = HashSet::new();
    let mut state_id = initial_state;
    let mut position: isize = 0;

    for _ in 0..steps {
        let state = states.iter().find(|x| x.id == state_id).unwrap();
        let value = tape.contains(&position);
        let action = if value {
            &state.when_one
        } else {
            &state.when_zero
        };
        let write_value = action.write > 0;
        if write_value {
            tape.insert(position);
        } else {
            tape.remove(&position);
        }
        position = position
            + (if action.go_direction == Direction::Left {
                -1
            } else {
                1
            });
        state_id = action.next_state;
    }
    let checksum = tape.len();
    println!("Part1 checksum: {}", checksum);
}

fn get_char_match(capture: &Captures<'_>, index: usize) -> char {
    capture.get(index).unwrap().as_str().chars().next().unwrap()
}

fn get_number_match(capture: &Captures<'_>, index: usize) -> usize {
    capture.get(index).unwrap().as_str().parse().unwrap()
}

fn get_direction_match(capture: &Captures<'_>, index: usize) -> Direction {
    match capture.get(index).unwrap().as_str() {
        "left" => Direction::Left,
        "right" => Direction::Right,
        x => panic!(format!("Unknown direction {:?}", x)),
    }
}
