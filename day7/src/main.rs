extern crate regex;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    let mut programs: Vec<&str> = vec![];
    let mut weights: HashMap<&str, usize> = HashMap::new();
    let mut parents: HashMap<&str, &str> = HashMap::new();
    let mut children_map: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in input.lines()  {
        // ibqmynm (252) -> oglcdgs, tkjofj, upurae, oypvhy, bzfkt, hdvcz, cfwxyl
        let re = Regex::new(r"([a-z]+) \((\d+)\)( -> )?([a-z ,]+)*").unwrap();

        for cap in re.captures_iter(line) {
            let name: &str = cap.get(1).unwrap().as_str();
            let weight: usize = cap.get(2).unwrap().as_str().parse().unwrap();
            let children: Vec<&str> = cap.get(4).map(|x| x.as_str().split(",").map(|child| child.trim()).collect()).unwrap_or(vec![]);

            weights.insert(name, weight);
            for child in &children {
                parents.insert(child, name);
            }
            children_map.insert(name, children);
            programs.push(name);
        }
    }

    let root = programs.iter().find(|&program| {
        match parents.get(program) {
            Some(_) => false,
            None => true
        }
    }).unwrap();
    println!("Root: {}", root);

    find_imbalance(root, &weights, &children_map);

    println!("Programs: {}", programs.len());
}

fn find_imbalance(root: &str, weights: &HashMap<&str, usize>, children_map: &HashMap<&str, Vec<&str>>) -> usize {
    let children: Vec<&str> = children_map.get(root).unwrap_or(&vec![]).to_vec();
    let weight = weights.get(root).unwrap();
    let mut total_weight: usize = *weight;

    let mut seen_child_weights = HashMap::new();
    let mut child_tower_weight_map = HashMap::new();
    for &node in &children {
        let child_weight = find_imbalance(node, weights, children_map);
        let child_weight_entry = seen_child_weights.entry(child_weight).or_insert(0);
        *child_weight_entry += 1;
        child_tower_weight_map.insert(node, child_weight);

        total_weight = total_weight + child_weight;
    }

    if seen_child_weights.len() > 1 {
        println!("Found imbalance: {:?}", seen_child_weights);
        for &child in &children {
            let child_weight = weights.get(child).unwrap();
            println!("Child {} weight: {}", child, child_weight);
            println!("Child {} tower weight: {}", child, child_tower_weight_map.get(child).unwrap());
        }
    }
    return total_weight;
}

