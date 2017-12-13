extern crate regex;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt").trim();
    // let input = include_str!("alt_input.txt");

    let mut range_by_depth: HashMap<usize, usize> = HashMap::new();
    let mut max_depth = 0usize;

    for line in input.lines() {
        let re = Regex::new(r"(\d+): (\d+)").unwrap();

        let capture = re.captures_iter(line).last().expect("Regex should match");
        let depth: usize = capture.get(1).unwrap().as_str().parse().unwrap();
        let range: usize = capture.get(2).unwrap().as_str().parse().unwrap();
        range_by_depth.insert(depth, range);
        if depth > max_depth {
            max_depth = depth;
        }
    }

    println!("score #1: {:?}", calculate_score(0, max_depth, &range_by_depth));

    for initial_time in 0.. {
        let score = calculate_score(initial_time, max_depth, &range_by_depth);
        if score == 0 {
            println!("Initial delay: {}", initial_time);
            break;
        }
    }
}

fn calculate_score(
    initial_time: usize,
    max_depth: usize,
    range_by_depth: &HashMap<usize, usize>
) -> usize {
    (0 .. max_depth + 1).map(|depth| {
        let layer_range: usize = match range_by_depth.get(&depth) {
            Some(range) => *range,
            None => return 0
        };
        let current_position = calculate_position(depth + initial_time, layer_range);
        if current_position == 0 {
            let mut score = depth * layer_range;
            if score < 1 {
                score = 1;
            }
            return score;
        }
        0
    }).fold(0, |x, y| x + y)
}

#[test]
fn test_calculate_score() {
    let max_depth = 6;
    let mut range_by_depth: HashMap<usize, usize> = HashMap::new();
    range_by_depth.insert(0, 3);
    range_by_depth.insert(1, 2);
    range_by_depth.insert(4, 4);
    range_by_depth.insert(6, 4);

    for i in 0..10 {
        assert!(calculate_score(i, max_depth, &range_by_depth) > 0, "i = {}", i);
    }
    assert_eq!(calculate_score(10, max_depth, &range_by_depth), 0);
}

fn calculate_position(time: usize, range: usize) -> usize {
    time % (2 * (range - 1))
}
