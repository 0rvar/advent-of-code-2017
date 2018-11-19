use std::collections::HashSet;

type Component = (usize, usize);

fn main() {
    let components = include_str!("input.txt")
        .lines()
        .map(|x| {
            let parts = x.split("/").collect::<Vec<_>>();
            let (a, b) = (parts[0], parts[1]);
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect::<Vec<Component>>();

    println!("Part 1: {}", strongest_bridge_strength(&components));
    println!("Part 2: {}", strongest_longest_bridge_strength(&components));
}

fn strongest_longest_bridge_strength(components: &[Component]) -> usize {
    let permutations = component_permutations(components);
    let max_length = permutations.iter().map(|x| x.len()).max().unwrap();
    permutations
        .iter()
        .filter(|x| x.len() == max_length)
        .map(|x| bridge_strength(&x))
        .max()
        .expect("wat")
}

fn strongest_bridge_strength(components: &[Component]) -> usize {
    let permutations = component_permutations(components);
    permutations
        .iter()
        .map(|x| bridge_strength(&x))
        .max()
        .expect("wat")
}

#[test]
fn test_strongest_bridge_strength() {
    let components = vec![
        (0, 2),
        (2, 2),
        (2, 3),
        (3, 4),
        (3, 5),
        (0, 1),
        (10, 1),
        (9, 10),
    ];
    assert_eq!(strongest_bridge_strength(&components), 31);
}

fn component_permutations(components: &[Component]) -> Vec<Vec<Component>> {
    component_permutations_permute(components, &vec![], 0, &HashSet::new())
}

fn component_permutations_permute(
    components: &[Component],
    current_bridge: &[Component],
    port: usize,
    used_components: &HashSet<usize>,
) -> Vec<Vec<Component>> {
    let mut result: Vec<Vec<Component>> = Vec::new();
    let valid_components = components
        .iter()
        .enumerate()
        .filter(|(index, &component)| {
            let (a, b) = component;
            (port == a || port == b) && !used_components.contains(index)
        })
        .collect::<Vec<_>>();
    for (component_index, build_with) in valid_components {
        let next_port = opposite_port(build_with, port);
        let next_used_components = {
            let mut u = used_components.clone();
            u.insert(component_index);
            u
        };
        let mut next_bridge = current_bridge
            .iter()
            .map(|x| x.clone())
            .collect::<Vec<Component>>();
        next_bridge.push(build_with.clone());
        result.push(next_bridge.clone());

        result.extend(component_permutations_permute(
            components,
            &next_bridge,
            next_port,
            &next_used_components,
        ));
    }
    result
}

fn bridge_strength(bridge: &[Component]) -> usize {
    bridge.iter().map(|(a, b)| a + b).sum()
}

#[test]
fn test_bridge_strength() {
    assert_eq!(bridge_strength(&vec![(0, 1), (10, 1), (9, 10)]), 31)
}

fn opposite_port(&(a, b): &Component, port: usize) -> usize {
    if a == port {
        b
    } else {
        a
    }
}
