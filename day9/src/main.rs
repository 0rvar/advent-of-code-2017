fn main() {
    let input = include_str!("input.txt").trim();
    let (score, removed_garbage) = total_score(input);

    println!("Total score: {}", score);
    println!("Removed garbage: {}", removed_garbage);
}

fn total_score(input: &str) -> (usize, usize) {
    let mut score = 0usize;
    let mut level = 0usize;
    let mut is_garbage = false;
    let mut is_negated = false;
    let mut removed_garbage = 0usize;

    for x in input.chars() {
        if !is_negated && is_garbage && x != '>' && x != '!' {
            removed_garbage += 1;
        }

        if is_negated {
            is_negated = false;
            continue;
        } else if x == '<' {
            is_garbage = true;
        } else if x == '>' {
            is_garbage = false;
        } else if x == '!' {
            is_negated = true;
        } else if !is_garbage {
            if x == '{' {
                level = level + 1;
                score = score + level;
            } else if x == '}' {
                level = level - 1;
            } else if x != ',' {
                panic!("Unexpected char '{}'", x);
            }
        }
    }

    (score, removed_garbage)
}
