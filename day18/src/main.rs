use std::collections::HashMap;

fn main() {
    let instructions = include_str!("input.txt").trim().lines().collect::<Vec<_>>();
    
    let mut registers: HashMap<char, isize> = HashMap::new();
    let mut position: usize = 0;

    let mut most_recently_played_sound: isize = 0;

    while position < instructions.len() {
        let instruction = instructions[position];
        let parts = instruction.split(' ').collect::<Vec<_>>();
        let op = parts[0];

        let arg1 = parse_reg_or_value(parts[1]);
        let arg2 = parts.get(2).map(|x| parse_reg_or_value(x));

        match op {
            "snd" => {
                most_recently_played_sound = get_value(&mut registers, &arg1);
            },
            "set" => {
                let target = get_register(&arg1);
                let value = get_value(&mut registers, &arg2.expect(&format!("Expected 2 args for {}", op)));
                registers.insert(target, value);
            },
            "add" => {
                let target = get_register(&arg1);
                let value = get_value(&mut registers, &arg2.expect(&format!("Expected 2 args for {}", op)));
                let reg = registers.entry(target).or_insert(0);
                *reg += value;
            },
            "mul" => {
                let target = get_register(&arg1);
                let value = get_value(&mut registers, &arg2.expect(&format!("Expected 2 args for {}", op)));
                let reg = registers.entry(target).or_insert(0);
                *reg *= value;
            },
            "mod" => {
                let target = get_register(&arg1);
                let value = get_value(&mut registers, &arg2.expect(&format!("Expected 2 args for {}", op)));
                let reg = registers.entry(target).or_insert(0);
                *reg = *reg % value;
            },
            "rcv" => {
                let value = get_value(&mut registers, &arg1);
                if value != 0 {
                    println!("rcv {}: Recovered frequency: {}", value, most_recently_played_sound);
                    break;
                }
            },
            "jgz" => {
                let value = get_value(&mut registers, &arg1);
                let offset = get_value(&mut registers, &arg2.expect(&format!("Expected 2 args for {}", op)));
                if value > 0 {
                    position = (position as isize + offset) as usize;
                    continue;
                }
            },
            _ => panic!(format!("Unknown op: {}", op))
        }

        position = position + 1;
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Argument {
    Register(char),
    Value(isize)
}

fn get_register(argument: &Argument) -> char {
    use Argument::*;
    match argument {
        &Register(r) => r,
        _ => panic!(format!("Expected {:?} to be register", argument))
    }
}

fn get_value(registers: &mut HashMap<char, isize>, argument: &Argument) -> isize {
    use Argument::*;
    match argument {
        &Register(r) => *registers.entry(r).or_insert(0),
        &Value(v) => v
    }
}

fn parse_reg_or_value(s: &str) -> Argument {
    match s.parse() {
        Ok(x) => Argument::Value(x),
        _ => Argument::Register(s.chars().last().unwrap())
    }
}