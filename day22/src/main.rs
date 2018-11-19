use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Direction {
    N,
    E,
    S,
    W,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum TurnDirection {
    Left,
    Right,
}

type Position = (isize, isize);

#[derive(Debug, PartialEq, Eq, Clone)]
enum NodeStatus {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

fn main() {
    let puzzle_input_raw = include_str!("input.txt").trim();
    let rows = puzzle_input_raw.lines().collect::<Vec<_>>();
    let num_rows = rows.len() as isize;

    let mut nodes: HashMap<Position, NodeStatus> = HashMap::new();
    for (row_index, row) in rows.iter().enumerate() {
        let centered_row_index = row_index as isize - (num_rows / 2);
        let num_columns = row.len() as isize;
        for (column_index, cell) in row.chars().enumerate() {
            let centered_column_index = column_index as isize - (num_columns / 2);
            if cell == '#' {
                nodes.insert(
                    (centered_row_index, centered_column_index),
                    NodeStatus::Infected,
                );
            }
        }
    }

    {
        let mut current_position: Position = (0, 0);
        let mut current_direction = Direction::N;
        let mut infections = 0;
        let mut infected_nodes = nodes.clone();
        for _ in 0..10000 {
            let current_node_is_infected = infected_nodes.contains_key(&current_position);
            let turn_to = if current_node_is_infected {
                TurnDirection::Right
            } else {
                TurnDirection::Left
            };
            current_direction = turn(current_direction, turn_to);
            if current_node_is_infected {
                infected_nodes.remove(&current_position);
            } else {
                infected_nodes.insert(current_position, NodeStatus::Infected);
                infections += 1;
            }
            current_position = move_in_direction(&current_position, &current_direction);
        }

        println!("Part 1. Infections after 10 000 bursts: {}", infections)
    }

    {
        let mut current_position: Position = (0, 0);
        let mut current_direction = Direction::N;
        let mut infections = 0;
        let mut nodes = nodes.clone();
        for _ in 0..10_000_000 {
            let node_status = nodes.get(&current_position).unwrap_or(&NodeStatus::Clean);
            current_direction = match node_status {
                NodeStatus::Clean => turn(current_direction, TurnDirection::Left),
                NodeStatus::Weakened => current_direction,
                NodeStatus::Infected => turn(current_direction, TurnDirection::Right),
                NodeStatus::Flagged => reverse_direction(current_direction),
            };
            match node_status {
                NodeStatus::Clean => {
                    nodes.insert(current_position, NodeStatus::Weakened);
                }
                NodeStatus::Weakened => {
                    nodes.insert(current_position, NodeStatus::Infected);
                    infections += 1;
                }
                NodeStatus::Infected => {
                    nodes.insert(current_position, NodeStatus::Flagged);
                }
                NodeStatus::Flagged => {
                    nodes.remove(&current_position);
                }
            }
            current_position = move_in_direction(&current_position, &current_direction);
        }

        println!("Part 2. Infections after 10_000_000 bursts: {}", infections)
    }
}

fn move_in_direction(&(row, column): &Position, dir: &Direction) -> Position {
    match dir {
        &Direction::N => (row - 1, column),
        &Direction::S => (row + 1, column),
        &Direction::E => (row, column + 1),
        &Direction::W => (row, column - 1),
    }
}

fn reverse_direction(direction: Direction) -> Direction {
    match direction {
        Direction::N => Direction::S,
        Direction::E => Direction::W,
        Direction::S => Direction::N,
        Direction::W => Direction::E,
    }
}

fn turn(direction: Direction, turn: TurnDirection) -> Direction {
    use self::TurnDirection::*;
    match direction {
        Direction::N => {
            if turn == Left {
                Direction::W
            } else {
                Direction::E
            }
        }
        Direction::E => {
            if turn == Left {
                Direction::N
            } else {
                Direction::S
            }
        }
        Direction::S => {
            if turn == Left {
                Direction::E
            } else {
                Direction::W
            }
        }
        Direction::W => {
            if turn == Left {
                Direction::S
            } else {
                Direction::N
            }
        }
    }
}
