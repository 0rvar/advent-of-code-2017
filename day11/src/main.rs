fn main() {
    let input = include_str!("input.txt").trim();

    let commands = input.split(",").collect::<Vec<_>>();
    let distance_from_origin = cube_distance(step_cube(&commands), (0, 0, 0));
    println!("Distance: {}", distance_from_origin);

    let mut max_distance = 0;
    for length in 1..commands.len() {
        let command_subset = &commands[0..length];
        let distance_from_origin = cube_distance(step_cube(command_subset), (0, 0, 0));
        if distance_from_origin > max_distance {
            max_distance = distance_from_origin;
        }
    }
    println!("Max distance: {}", max_distance);
}

fn cube_distance((ax, ay, az): (isize, isize, isize), (bx, by, bz): (isize, isize, isize)) -> isize {
    ((ax - bx).abs() + (ay - by).abs() + (az - bz).abs()) / 2
}

#[test]
fn test_cube_distance() {
    assert_eq!(cube_distance((-3, 1, 2), (0, 0, 0)), 3);
    assert_eq!(cube_distance((1, -4, 3), (0, 0, 0)), 4);
}

fn step_cube(commands: &[&str]) -> (isize, isize, isize) {
    let mut x = 0;
    let mut y = 0;
    let mut z = 0;

    for command in commands {
        match command {
            &"n" => {
                y += 1;
                z -= 1;
            },
            &"s" => {
                y -= 1;
                z += 1;
            },
            &"nw" => {
                y += 1;
                x -= 1;
            },
            &"se" => {
                y -= 1;
                x += 1;
            },
            &"ne" => {
                z -= 1;
                x += 1;
            },
            &"sw" => {
                z += 1;
                x -= 1;
            },
            x => {
                panic!("Unknown direction '{}'", x);
            }
        }
    }

    (x, y, z)
}

#[test]
fn test_step_cube() {
    assert_eq!(step_cube(&vec!["n", "nw", "sw", "s"]), (-2, 1, 1));
    assert_eq!(step_cube(&vec!["n", "ne", "se", "s"]), (2, -1, -1));
}
