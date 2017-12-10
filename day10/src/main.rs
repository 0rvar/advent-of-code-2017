fn main() {
    let input = include_str!("input.txt");
    let sizes: Vec<usize> = input.trim().split(",").map(|x| x.parse().unwrap()).collect::<Vec<_>>();

    let collection = (0..256).collect::<Vec<_>>();
    let hashed_collection = hash(&collection, &sizes, 1);
    println!("{:?}", hashed_collection[0..2].iter().collect::<Vec<_>>());

    let puzzle_hash = knot_hash(input.trim());
    println!("{}", puzzle_hash);
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
    dense_hash_hex.join("")
}

#[test]
fn test_know_hash() {
    assert_eq!(knot_hash(""), "a2582a3a0e66e6e86e3812dcb672a272");
    assert_eq!(knot_hash("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
    assert_eq!(knot_hash("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
    assert_eq!(knot_hash("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
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