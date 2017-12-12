extern crate regex;
use regex::Regex;
use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    // let input = include_str!("alt_input.txt");

    let mut programs: Vec<(usize, Vec<usize>)> = Vec::new();

    for line in input.lines()  {
        let re = Regex::new(r"(\d+) <-> ([\d+\W,]+)").unwrap();

        let capture = re.captures_iter(line).last().expect("Regex should match");
        let program: usize = capture.get(1).unwrap().as_str().parse().unwrap();
        let connections: Vec<usize> = capture.get(2)
            .map(|x| x
                .as_str()
                .split(", ")
                .map(|child| child.trim())
                .filter(|child| child.len() > 0)
                .map(|child|
                    child
                        .trim()
                        .parse()
                        .unwrap_or_else(|v| panic!(format!("Failed to parse usize {}: {}", child, v))))
                .collect()
            )
            .unwrap_or_else(|| vec![]);
        programs.push((program, connections));
    }

    let mut groups: Vec<(usize, HashSet<usize>)> = Vec::new();

    while programs.len() > 0 {
        let group_leader = programs[0].0;
        let mut group = HashSet::new();
        group.insert(group_leader);

        let mut last_programs_size = 0;

        while last_programs_size != programs.len() {
            last_programs_size = programs.len();

            let mut next_programs = Vec::new();

            for (program, connections) in programs {
                if group.contains(&program) ||
                    connections.iter().any(|c| group.contains(c)) {
                    group.insert(program);
                    for c in connections { group.insert(c); }
                } else {
                    next_programs.push((program, connections));
                }
            }
                

            programs = next_programs;
        }

        println!("{} group size: {}", group_leader, group.len());
        groups.push((group_leader, group));
    }

    println!("Group count: {}", groups.len())
}