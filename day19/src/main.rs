#![feature(exclusive_range_pattern)]

use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Direction {
    N, E, S, W
}

type Position = (usize, usize);

#[derive(Debug, PartialEq, Eq)]
enum Entity {
    None,
    Horizontal,
    Vertical,
    Junction,
    Letter(char)
}

fn main() {
    let input = include_str!("input.txt").lines().collect::<Vec<&str>>();

    let mut map: HashMap<Position, Entity> = HashMap::new();
    for (row, &line) in input.iter().enumerate() {
        for (column, character) in line.chars().enumerate() {
            map.insert((row, column), parse_entity(character));
        }
    }

    let mut current_position: (usize, usize) = (
        0,
        (0usize..).find(|&column| map.get(&(0, column)) == Some(&Entity::Vertical)).unwrap()
    );
    let mut current_direction = Direction::S;
    let mut letters: Vec<char> = vec![];
    let mut num_steps = 0;

    loop {
        let current_entity = map.get(&current_position).unwrap_or_else(|| &Entity::None);
        // println!("{:?} = '{:?}', moving {:?}", current_position, current_entity, current_direction);
        match current_entity {
            &Entity::Horizontal | &Entity::Vertical => {
                current_position = move_in_direction(&current_position, &current_direction);
                num_steps += 1;
            },
            &Entity::Junction => {
                let options = vec![
                    (Direction::N, Entity::Vertical),
                    (Direction::S, Entity::Vertical),
                    (Direction::W, Entity::Horizontal),
                    (Direction::E, Entity::Horizontal)
                ];
                let &(ref new_direction, _) = options.iter()
                    .find(|&&(ref dir, ref allowed_entity)| {
                        if dir == &opposite_direction(&current_direction) {
                            return false
                        }

                        let pos: Position = move_in_direction(&current_position, &dir);
                        let entity: &Entity = map.get(&pos).unwrap_or_else(|| &Entity::None);

                        entity == allowed_entity
                    })
                    .expect(&format!("Should have path at {:?} going {:?}", current_position, current_direction));
                current_direction = new_direction.clone();
                current_position = move_in_direction(&current_position, &current_direction);
                num_steps += 1;
            },
            &Entity::Letter(c) => {
                letters.push(c);
                current_position = move_in_direction(&current_position, &current_direction);
                num_steps += 1;
            },
            &Entity::None => {
                break;
            }
        }
    }

    println!("Seen letters: {}", letters.iter().collect::<String>());
    println!("Num steps: {}", num_steps);
}

fn opposite_direction(dir: &Direction) -> Direction {
    match dir {
        &Direction::N => Direction::S,
        &Direction::S => Direction::N,

        &Direction::W => Direction::E,
        &Direction::E => Direction::W
    }
}

fn move_in_direction(&(row, column): &Position, dir: &Direction) -> Position {
    match dir {
        &Direction::N => (row-1, column),
        &Direction::S => (row+1, column),
        &Direction::E => (row, column+1),
        &Direction::W => (row, column-1),
    }
}

fn parse_entity(c: char) -> Entity {
    match c {
        ' ' => Entity::None,
        '|' => Entity::Vertical,
        '-' => Entity::Horizontal,
        '+' => Entity::Junction,
        'A'..'Z' => Entity::Letter(c),
        _ => panic!(format!("Unknown map entity '{}'", c))
    }
}
