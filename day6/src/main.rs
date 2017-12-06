fn main() {
    let input = "4	1	15	12	0	9	9	5	5	8	7	3	14	5	12	3";
    let source: Vec<usize> = input
        .split('\t')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();

    use std::collections::HashMap;
    let mut unique_configurations: HashMap<String, usize> = HashMap::new();

    let mut memory_banks = source.clone();

    unique_configurations.insert(join(&memory_banks), 0);

    let mut redistributions = 0;
    loop {
        let mut new_memory_banks = memory_banks.clone();
        let mut index: usize = get_max_index(&new_memory_banks);
        let mut stack: usize = new_memory_banks[index];
        new_memory_banks[index] = 0;
        while stack > 0 {
            index = index + 1;
            if index >= new_memory_banks.len() {
                index = 0;
            }
            new_memory_banks[index] += 1;
            stack -= 1;
        }
        redistributions = redistributions + 1;
        memory_banks = new_memory_banks;

        let entry = unique_configurations.entry(join(&memory_banks)).or_insert(redistributions);
        if *entry < redistributions {
            println!("Infinite loop length: {}", redistributions - *entry);
            break;
        }
    }

    println!("Redistributions: {}", redistributions);
}

fn join(array: &[usize]) -> String {
    array.iter()
        .fold(String::new(), |acc, num| {
            acc + " " + &num.to_string()
        })
}

fn get_max_index(collection: &[usize]) -> usize {
    let mut max_value = collection[0];
    let mut max_index = 0;
    for (index, &value) in collection.iter().enumerate() {
        if value > max_value {
            max_value = value;
            max_index = index;
        }
    }
    max_index
}

#[test]
fn should_return_correct_max_index() {
    assert_eq!(get_max_index(&[1, 3, 2, 3]), 1);
}
