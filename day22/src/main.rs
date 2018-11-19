use std::collections::HashSet;

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

fn main() {
    let puzzle_input_raw = include_str!("input.txt").trim();
    let rows = puzzle_input_raw.lines().collect::<Vec<_>>();
    let num_rows = rows.len() as isize;

    let mut infected_nodes: HashSet<Position> = HashSet::new();
    for (row_index, row) in rows.iter().enumerate() {
        let centered_row_index = row_index as isize - (num_rows / 2);
        let num_columns = row.len() as isize;
        for (column_index, cell) in row.chars().enumerate() {
            let centered_column_index = column_index as isize - (num_columns / 2);
            if cell == '#' {
                infected_nodes.insert((centered_row_index, centered_column_index));
            }
        }
    }

    let mut current_position: Position = (0, 0);
    let mut current_direction = Direction::N;
    let mut infections = 0;
    for _ in 0..10000 {
        let current_node_is_infected = infected_nodes.contains(&current_position);
        let turn_to = if current_node_is_infected {
            TurnDirection::Right
        } else {
            TurnDirection::Left
        };
        current_direction = turn(current_direction, turn_to);
        if current_node_is_infected {
            infected_nodes.remove(&current_position);
        } else {
            infected_nodes.insert(current_position);
            infections += 1;
        }
        current_position = move_in_direction(&current_position, &current_direction);
    }

    println!("Infections after 10 000 bursts: {}", infections)
}

fn move_in_direction(&(row, column): &Position, dir: &Direction) -> Position {
    match dir {
        &Direction::N => (row - 1, column),
        &Direction::S => (row + 1, column),
        &Direction::E => (row, column + 1),
        &Direction::W => (row, column - 1),
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
