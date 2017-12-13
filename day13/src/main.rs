extern crate regex;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt").trim();
    // let input = include_str!("alt_input.txt");


    let mut layer_depths: Vec<usize> = Vec::new();
    let mut range_by_depth: HashMap<usize, usize> = HashMap::new();
    let mut max_depth = 0usize;

    for line in input.lines() {
        let re = Regex::new(r"(\d+): (\d+)").unwrap();

        let capture = re.captures_iter(line).last().expect("Regex should match");
        let depth: usize = capture.get(1).unwrap().as_str().parse().unwrap();
        let range: usize = capture.get(2).unwrap().as_str().parse().unwrap();
        layer_depths.push(depth);
        range_by_depth.insert(depth, range);
        if depth > max_depth {
            max_depth = depth;
        }
    }

    // println!("layer_depths: {:?}", layer_depths);
    // println!("range_by_depth: {:?}", range_by_depth);
    // println!("max_depth: {:?}", max_depth);
    // println!("range: {:?}", (0 .. max_depth + 1).collect::<Vec<_>>());
    
    let scores = (0 .. max_depth + 1).map(|depth| {
        let layer_range = match range_by_depth.get(&depth) {
            Some(range) => range,
            None => return 0
        };
        let current_position = calculate_position(depth, *layer_range);
        if current_position == 0 {
            return depth * layer_range;
        }
        0
    });
    let score = scores.fold(0, |x, y| x + y);
    println!("score: {:?}", score);
}

fn calculate_position(time: usize, range: usize) -> usize {
    let mut downwards = true;
    let mut position = 0;
    for _ in 1 .. time + 1 {
        if position == 0 && !downwards {
            downwards = true;
        } else if position >= range - 1 && downwards {
            downwards = false;
        }

        if downwards {
            position += 1;
        } else {
            position -= 1;
        }
    }
    position
}

#[test]
fn test_calculate_position_with_range_2() {
    assert_eq!(calculate_position(0, 2), 0);
    assert_eq!(calculate_position(1, 2), 1);
    assert_eq!(calculate_position(2, 2), 0);
    assert_eq!(calculate_position(3, 2), 1);
}


#[test]
fn test_calculate_position_with_range_3() {
    assert_eq!(calculate_position(0, 3), 0);
    assert_eq!(calculate_position(1, 3), 1);
    assert_eq!(calculate_position(2, 3), 2);
    assert_eq!(calculate_position(3, 3), 1);
    assert_eq!(calculate_position(4, 3), 0);
}
