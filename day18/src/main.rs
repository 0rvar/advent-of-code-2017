use std::collections::HashMap;
use std::collections::VecDeque;

fn apply(regs: &mut HashMap<char, isize>, arg1: &Argument, arg2: Option<Argument>, applier: fn(&mut isize, isize)) {
    let target = get_register(&arg1);
    let arg2_unwrapped: Argument = arg2.expect("Expected 2 args for math op");
    let value = get_value(regs, &arg2_unwrapped);
    let reg = regs.entry(target).or_insert(0);
    applier(reg, value);
}


fn main() {
    let instructions = include_str!("input.txt").trim().lines().collect::<Vec<_>>();
    //let instructions = include_str!("input_ex.txt").trim().lines().collect::<Vec<_>>();
    
    {
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
                "set" => apply(&mut registers, &arg1, arg2, |reg, value| *reg = value),
                "add" => apply(&mut registers, &arg1, arg2, |reg, value| *reg += value),
                "mul" => apply(&mut registers, &arg1, arg2, |reg, value| *reg *= value),
                "mod" => apply(&mut registers, &arg1, arg2, |reg, value| *reg = *reg % value),
                "rcv" => {
                    let value = get_value(&mut registers, &arg1);
                    if value != 0 {
                        println!("rcv {}: Recovered frequency: {}", value, most_recently_played_sound);
                        break;
                    }
                },
                "snd" => {
                    most_recently_played_sound = get_value(&mut registers, &arg1);
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

    {
        let mut registers_a: HashMap<char, isize> = HashMap::new();
        registers_a.insert('p', 0);
        let mut registers_b: HashMap<char, isize> = HashMap::new();
        registers_b.insert('p', 1);

        let mut sends_from_a: VecDeque<isize> = VecDeque::new();
        let mut sends_from_b: VecDeque<isize> = VecDeque::new();
        let mut a_receiving_to: Option<char> = None;
        let mut b_receiving_to: Option<char> = None;
        let mut num_sends_from_a = 0;
        let mut num_sends_from_b = 0;
        
        let mut position_a: usize = 0;
        let mut position_b: usize = 0;

        while position_a < instructions.len() && position_b < instructions.len() {
            if a_receiving_to != None && sends_from_b.len() == 0 && b_receiving_to != None && sends_from_a.len() == 0 {
                println!("Deadlock");
                break;
            }

            match a_receiving_to {
                Some(reg) => {
                    match sends_from_b.pop_front() {
                        Some(value) => {
                            apply(&mut registers_a, &Argument::Register(reg), Some(Argument::Value(value)), |reg, value| *reg = value);
                            a_receiving_to = None;
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
            match b_receiving_to {
                Some(reg) => {
                    match sends_from_a.pop_front() {
                        Some(value) => {
                            apply(&mut registers_b, &Argument::Register(reg), Some(Argument::Value(value)), |reg, value| *reg = value);
                            b_receiving_to = None;
                        },
                        _ => {}
                    }
                },
                _ => {}
            }


            if a_receiving_to == None {
                let instruction_a = instructions[position_a];
                let parts = instruction_a.split(' ').collect::<Vec<_>>();
                let op = parts[0];

                let arg1 = parse_reg_or_value(parts[1]);
                let arg2 = parts.get(2).map(|x| parse_reg_or_value(x));

                match op {
                    "set" => apply(&mut registers_a, &arg1, arg2, |reg, value| *reg = value),
                    "add" => apply(&mut registers_a, &arg1, arg2, |reg, value| *reg += value),
                    "mul" => apply(&mut registers_a, &arg1, arg2, |reg, value| *reg *= value),
                    "mod" => apply(&mut registers_a, &arg1, arg2, |reg, value| *reg = *reg % value),
                    "rcv" => {
                        let reg = get_register(&arg1);
                        a_receiving_to = Some(reg);
                    },
                    "snd" => {
                        let value = get_value(&mut registers_a, &arg1);
                        sends_from_a.push_back(value);
                        num_sends_from_a += 1;
                    },
                    "jgz" => {
                        let value = get_value(&mut registers_a, &arg1);
                        let offset = get_value(&mut registers_a, &arg2.expect(&format!("Expected 2 args for {}", op)));
                        if value > 0 {
                            position_a = (position_a as isize + offset) as usize;
                            continue;
                        }
                    },
                    _ => panic!(format!("Unknown op: {}", op))
                }

                position_a = position_a + 1;
            }

            if b_receiving_to == None {
                let instruction_b = instructions[position_b];
                let parts = instruction_b.split(' ').collect::<Vec<_>>();
                let op = parts[0];

                let arg1 = parse_reg_or_value(parts[1]);
                let arg2 = parts.get(2).map(|x| parse_reg_or_value(x));

                match op {
                    "set" => apply(&mut registers_b, &arg1, arg2, |reg, value| *reg = value),
                    "add" => apply(&mut registers_b, &arg1, arg2, |reg, value| *reg += value),
                    "mul" => apply(&mut registers_b, &arg1, arg2, |reg, value| *reg *= value),
                    "mod" => apply(&mut registers_b, &arg1, arg2, |reg, value| *reg = *reg % value),
                    "rcv" => {
                        let reg = get_register(&arg1);
                        b_receiving_to = Some(reg);
                    },
                    "snd" => {
                        let value = get_value(&mut registers_b, &arg1);
                        sends_from_b.push_back(value);
                        num_sends_from_b += 1;
                    },
                    "jgz" => {
                        let value = get_value(&mut registers_b, &arg1);
                        let offset = get_value(&mut registers_b, &arg2.expect(&format!("Expected 2 args for {}", op)));
                        if value > 0 {
                            position_b = (position_b as isize + offset) as usize;
                            continue;
                        }
                    },
                    _ => panic!(format!("Unknown op: {}", op))
                }

                position_b = position_b + 1;
            }
        }
        println!("Sends from a: {}", num_sends_from_a);
        println!("Sends from b: {}", num_sends_from_b);
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