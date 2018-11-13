#![feature(slice_patterns)]

fn main() {
    let puzzle_input_raw = include_str!("input.txt").trim();

    let mut rules: Vec<(&str, &str)> = Vec::new();
    
    for line in puzzle_input_raw.lines() {
        let (input, output) = match line.split(" => ").collect::<Vec<_>>()[..] {
            [input, output] => (input, output),
            _ => panic!(format!("Could not parse {}", line))
        };
        rules.push((input, output));
    }

    let start_grid = ".#./..#/###";
    let mut grid = start_grid.to_string();

    for i in 0..5 {
        let squares = to_squares(&grid);

        let next_squares = squares; // TODO

        grid = from_squares(&next_squares);
    }
}

fn to_squares(grid: &str) -> Vec<String> {
    let rows = grid.split("/").collect::<Vec<_>>();
    let size = rows.len();

    let square_size = if size % 3 == 0 { 3 } else { 2 };
    let grid_size = size / square_size;
    let mut squares = Vec::new();

    for square_row in 0..grid_size {
        for square_column in 0..grid_size {
            let mut square = String::new();
            for offset_row in 0..square_size {
                for offset_column in 0..square_size {
                    square.push(rows[square_row * square_size + offset_row].chars().nth(square_column * square_size + offset_column).unwrap());
                }
                square.push('/');
            }
            square.pop();
            squares.push(square);
        }
    }

    squares
}

#[test]
fn test_to_squares() {
    assert_eq!(to_squares(".##./#.##/##../..##"), vec![
        ".#/#.".to_string(),
        "#./##".to_string(),
        "##/..".to_string(),
        "../##".to_string()
    ]);
}

fn from_squares(squares: &[String]) -> String {
    let size = (squares.len() as f64).sqrt() as usize;
    let square_size = squares[0].split("/").count();
    let mut chars: Vec<char> = Vec::new();

    let grid_char_size = size * square_size;

    for row in 0..grid_char_size {
        for column in 0..grid_char_size {
            let square_row = row / square_size;
            let square_column = column / square_size;
            let square_row_offset = row % square_size;
            let square_column_offset = column % square_size;

            let square_index = square_row * size + square_column;
            let square_offset = square_row_offset * square_size + square_column_offset;
            let c = squares[square_index].chars().filter(|&x| x != '/').nth(square_offset).unwrap();
            chars.push(c);
        }
        chars.push('/');
    }
    chars.pop();

    chars.iter().collect::<String>()
}

#[test]
fn test_from_squares() {
    assert_eq!(from_squares(&vec![
        ".#/#.".to_string(),
        "#./##".to_string(),
        "##/..".to_string(),
        "../##".to_string()
    ]), ".##./#.##/##../..##".to_string());
}


fn rotate(square: String, steps: u8) -> String {
    if steps == 0 {
        return square;
    }
    if steps >= 2 {
        let mut half_rot_vec = square.split("/")
            .map(|x| x.chars().rev().collect::<String>())
            .collect::<Vec<_>>();
        half_rot_vec.reverse();

        return rotate(join(&half_rot_vec), steps - 2);
    }
    let mut chars: Vec<char> = Vec::new();
    let rows = square.split("/").collect::<Vec<&str>>();
    let size = rows.len();
    for i in 0..size {
        for j in 0..size {
            chars.push(rows[size - 1 - j].chars().nth(i).unwrap());
        }
        chars.push('/');
    }
    chars.pop();

    chars.iter().collect::<String>()
}

#[test]
fn test_rotate() {
    assert_eq!(rotate(".#./..#/.##".to_string(), 1), ".../#.#/##.".to_string());
    assert_eq!(rotate(".#./..#/.##".to_string(), 2), "##./#../.#.".to_string());
    assert_eq!(rotate(".#./..#/.##".to_string(), 3), ".##/#.#/...".to_string());
}

fn flip(square: &str) -> String {
    let reversed = square.split("/")
        .map(|x| x.chars().rev().collect::<String>())
        .collect::<Vec<_>>();
    join(&reversed)
}

#[test]
fn test_flip() {
    assert_eq!(flip(".#/#."), "#./.#".to_string());
}

fn join<T: ToString>(square_split: &[T]) -> String
{
    let mut joined = square_split.iter()
        .fold(String::new(), |acc, x| acc + &x.to_string() + "/");
    joined.pop();
    joined
}
