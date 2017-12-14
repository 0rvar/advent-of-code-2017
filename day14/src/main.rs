use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let input = "jxqlasbh";
    // let input = "flqrgnkx";

    let mut grid: Vec<usize> = Vec::new();

    for row in 0..128 {
        let row_input = format!("{}-{}", input, row);
        let row_hash = knot_hash(&row_input);
        grid.extend(row_hash.chars().map(|c| match c { '1' => 1, _ => 0 }));
    }
    
    let sum = grid.iter().filter(|&&c| c == 1usize).count();
    println!("{}", sum);

    let mut seizures: HashMap<usize, usize> = HashMap::new();
    let index = |row: usize, column: usize| row * 128 + column;
    let mut block_counter = 2;
    loop {
        let mut changed = false;

        for row in 0..128 {
            for column in 0..128 {
                let mut cell_value = grid[index(row, column)];
                if cell_value == 0 {
                    continue;
                }
                if cell_value == 1 {
                    cell_value = block_counter;
                    block_counter += 1;
                } else if let Some(seized_by) = seizures.get(&cell_value) {
                    cell_value = *seized_by;
                }
                grid[index(row, column)] = cell_value;

                let neighbor_deltas: Vec<(isize, isize)> = vec![
                    (1, 0),
                    (0, 1)
                ];
                for (delta_row, delta_column) in neighbor_deltas {
                    let (n_row, n_column) = (
                        (row as isize + delta_row),
                        (column as isize + delta_column)
                    );
                    if n_row < 0 || n_row >= 128 || n_column < 0 || n_column >= 128 {
                        continue;
                    }
                    let neighbor_index = index(n_row as usize, n_column as usize);
                    let neighbor_value = grid[neighbor_index];
                    if neighbor_value == 0 || neighbor_value == cell_value {
                        continue;
                    }
                    if neighbor_value == 1 {
                        grid[neighbor_index] = cell_value;
                        changed = true;
                    } else {
                        if let None = seizures.get(&neighbor_value) {
                            seizures.insert(neighbor_value, cell_value);
                        }
                        changed = true;
                    }
                }
            }
        }

        if !changed {
            break;
        }
    }

    let mut block_set = HashSet::new();
    for x in grid {
        if x > 0 {
            block_set.insert(x);
        }
    }
    println!("{} blocks", block_set.len());
}



fn knot_hash(input: &str) -> String {
    let mut sizes = input.as_bytes().iter().map(|x| *x as usize).collect::<Vec<usize>>();
    sizes.extend([17, 31, 73, 47, 23].iter());

    let collection = (0..256).collect::<Vec<_>>();
    let hashed_collection = hash(&collection, &sizes, 64);

    let dense_hash = hashed_collection.chunks(16).map(|block| {
        block.iter().fold(0, |acc, &x| acc ^ x)
    }).collect::<Vec<usize>>();


    let dense_hash_hex = dense_hash.iter().map(|x| format!("{:02x}", *x as u8)).collect::<Vec<String>>();

    let dense_hash_binary = dense_hash_hex.join("").chars().map(|x: char| {
        let digit: u8 = u8::from_str_radix(&x.to_string(), 16).unwrap();
        let binary = format!("{:04b}", digit);

        binary
    }).collect::<Vec<String>>();

    dense_hash_binary.join("")
}


fn hash(collection: &[usize], sizes: &[usize], rounds: usize) -> Vec<usize> {
    let mut skip = 0;
    let mut index = 0;
    let mut current_collection: Vec<usize> = collection.to_vec();
    
    for _ in 0..rounds {
        for &size in sizes {
            current_collection = reverse_wrapping_subset(&current_collection, index, size);
            index = (index + size + skip) % current_collection.len();
            skip = skip + 1;
        }
    }

    current_collection
}


fn reverse_wrapping_subset(list: &[usize], start: usize, length: usize) -> Vec<usize> {
    let (prefix, sequence_and_postfix) = list.split_at(start);
    if sequence_and_postfix.len() > length {
        let (sequence, postfix) = sequence_and_postfix.split_at(length);

        let mut sorted_sequence: Vec<usize> = sequence.to_vec();
        sorted_sequence.reverse();

        let mut result = prefix.to_vec();
        result.append(&mut sorted_sequence);
        result.extend(postfix);
        result
    } else {
        let wrapped_sequence_length = length - sequence_and_postfix.len();
        let (sequence_tail, prefix) = prefix.split_at(wrapped_sequence_length);

        let mut sequence = sequence_and_postfix.to_vec();
        sequence.extend(sequence_tail);
        sequence.reverse();

        let (sequence_tail, sequence_init) = sequence.split_at(sequence.len() - wrapped_sequence_length);

        let mut result = sequence_init.to_vec();
        result.extend(prefix);
        result.extend(sequence_tail);

        result
    }
}

#[test]
fn test_wrapping() {
    let initial_state = vec![0, 1, 2, 3, 4];
    let reversed = reverse_wrapping_subset(&initial_state, 0, 3);
    assert_eq!(reversed, vec![2, 1, 0, 3, 4]);

    let reversed = reverse_wrapping_subset(&reversed, 3, 4);
    assert_eq!(reversed, vec![4, 3, 0, 1, 2]);

    let reversed = reverse_wrapping_subset(&reversed, 1, 5);
    assert_eq!(reversed, vec![3, 4, 2, 1, 0]);
}