fn main() {
    let input = include_str!("input.txt");
    let sizes: Vec<usize> = input.trim().split(",").map(|x| x.parse().unwrap()).collect::<Vec<_>>();

    let mut collection = (0..256).collect::<Vec<_>>();

    let mut skip = 0;
    let mut index = 0;
    for size in sizes {
        collection = reverse_wrapping_subset(&collection, index, size);
        index = (index + size + skip) % collection.len();
        skip = skip + 1;
    }

    println!("{:?}", collection);
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