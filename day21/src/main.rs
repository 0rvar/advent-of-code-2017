#![feature(slice_patterns)]

fn main() {
    let puzzle_input_raw = include_str!("input.txt").trim();

    let mut rules: Vec<(&str, &str)> = Vec::new();

    for line in puzzle_input_raw.lines() {
        let (input, output) = match line.split(" => ").collect::<Vec<_>>()[..] {
            [input, output] => (input, output),
            _ => panic!(format!("Could not parse {}", line)),
        };
        rules.push((input, output));
    }

    let start_grid = ".#./..#/###";
    let mut grid = start_grid.to_string();

    for i in 1..19 {
        let squares = to_squares(&grid);

        let next_squares = squares
            .iter()
            .map(|square| {
                for rule in &rules {
                    let (search, replace) = rule;
                    if match_rule(search, square) {
                        return replace.to_string();
                    }
                }
                // no match
                square.to_string()
            })
            .collect::<Vec<_>>();

        grid = from_squares(&next_squares);

        println!(
            "{} iterations, {} on",
            i,
            grid.chars().filter(|&x| x == '#').count()
        )
    }
}

fn match_rule(search: &str, square: &str) -> bool {
    return search == square
        || rotate(search, 1) == square
        || rotate(search, 2) == square
        || rotate(search, 3) == square
        || flip(search) == square
        || rotate(&flip(search), 1) == square
        || rotate(&flip(search), 2) == square
        || rotate(&flip(search), 3) == square;
}

fn print_grid(grid: &str) {
    println!("===============");
    for row in grid.split("/").collect::<Vec<_>>() {
        println!("    {}", row);
    }
    println!();
}

fn to_squares(grid: &str) -> Vec<String> {
    let rows = grid.split("/").collect::<Vec<_>>();
    let size = rows.len();

    let square_size = if size % 2 == 0 { 2 } else { 3 };
    let grid_size = size / square_size;
    let mut squares = Vec::new();

    for square_row in 0..grid_size {
        for square_column in 0..grid_size {
            let mut square = String::new();
            for offset_row in 0..square_size {
                for offset_column in 0..square_size {
                    square.push(
                        rows[square_row * square_size + offset_row]
                            .chars()
                            .nth(square_column * square_size + offset_column)
                            .unwrap(),
                    );
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
fn test_to_squares_size_2() {
    assert_eq!(
        to_squares(".##./#.##/##../..##"),
        vec![
            ".#/#.".to_string(),
            "#./##".to_string(),
            "##/..".to_string(),
            "../##".to_string()
        ]
    );
}
#[test]
fn test_to_squares_size_3() {
    assert_eq!(
        to_squares("##.##./#..#../....../##.##./#..#../......"),
        vec![
            "##/#.".to_string(),
            ".#/.#".to_string(),
            "#./..".to_string(),
            "../##".to_string(),
            "../.#".to_string(),
            "../#.".to_string(),
            "#./..".to_string(),
            ".#/..".to_string(),
            "../..".to_string(),
        ]
    )
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
            let c = squares[square_index]
                .chars()
                .filter(|&x| x != '/')
                .nth(square_offset)
                .unwrap();
            chars.push(c);
        }
        chars.push('/');
    }
    chars.pop();

    chars.iter().collect::<String>()
}

#[test]
fn test_from_squares() {
    assert_eq!(
        from_squares(&vec![
            ".#/#.".to_string(),
            "#./##".to_string(),
            "##/..".to_string(),
            "../##".to_string()
        ]),
        ".##./#.##/##../..##".to_string()
    );
}

fn rotate(square: &str, steps: u8) -> String {
    if steps == 0 {
        return square.to_string();
    }
    if steps >= 2 {
        let mut half_rot_vec = square
            .split("/")
            .map(|x| x.chars().rev().collect::<String>())
            .collect::<Vec<_>>();
        half_rot_vec.reverse();

        return rotate(&join_square_to_string(&half_rot_vec), steps - 2);
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
    assert_eq!(rotate(".#./..#/.##", 1), ".../#.#/##.");
    assert_eq!(rotate(".#./..#/.##", 2), "##./#../.#.");
    assert_eq!(rotate(".#./..#/.##", 3), ".##/#.#/...");
}

fn flip(square: &str) -> String {
    let reversed = square
        .split("/")
        .map(|x| x.chars().rev().collect::<String>())
        .collect::<Vec<_>>();
    join_square_to_string(&reversed)
}

#[test]
fn test_flip() {
    assert_eq!(flip(".#/#."), "#./.#".to_string());
    assert_eq!(flip("../#."), "../.#".to_string());
    assert_eq!(flip("#../..."), "..#/...".to_string());
}

fn join_square_to_string<T: ToString>(square_split: &[T]) -> String {
    square_split.iter().fold(String::new(), |acc, x| {
        let separator = if acc.len() > 0 { "/" } else { "" };
        acc + separator + &x.to_string()
    })
}
