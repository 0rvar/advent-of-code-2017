extern crate regex;
use regex::Regex;

use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    let mut values: HashMap<&str, isize> = HashMap::new();
    let mut max_value_ever = 0;

    for line in input.lines()  {
        let re = Regex::new(r"([a-z]+) (inc|dec) ([\d\-]+) if ([a-z]+) ([<>=!]+) ([\d\-]+)").unwrap();

        for cap in re.captures_iter(line) {
            let register_name: &str = cap.get(1).unwrap().as_str();
            let inc_or_dec: &str = cap.get(2).unwrap().as_str();
            let delta: isize = cap.get(3).unwrap().as_str().parse().unwrap();

            let comparison_register_name: &str = cap.get(4).unwrap().as_str();
            let comparison_operator: &str = cap.get(5).unwrap().as_str();
            let comparison_value: isize = cap.get(6).unwrap().as_str().parse().unwrap();

            let predicate_matches = {
                let comparison_register = values.entry(comparison_register_name).or_insert(0);
                let matches = match comparison_operator {
                    "==" => *comparison_register == comparison_value,
                    ">=" => *comparison_register >= comparison_value,
                    ">" => *comparison_register > comparison_value,
                    "<=" => *comparison_register <= comparison_value,
                    "<" => *comparison_register < comparison_value,
                    "!=" => *comparison_register != comparison_value,
                    x => panic!(format!("Unknown operator {}", x))
                };
                matches
            };
            
            let register = values.entry(register_name).or_insert(0);
            if predicate_matches {
                match inc_or_dec {
                    "inc" => { *register += delta },
                    "dec" => { *register -= delta },
                    x => panic!("Unknown inc/dec value {}", x)
                };
                if *register > max_value_ever {
                    max_value_ever = *register;
                }
                
            }
        }
    }

    if let Some((largest_register, largest_register_value)) = values.iter().max_by_key(|&(_reg, value)| value) {
        println!("{}, {}", &largest_register, &largest_register_value);
    }
    println!("Max value ever: {}", max_value_ever);

}
