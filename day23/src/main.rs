use std::collections::HashMap;

fn main() {
    let instructions = include_str!("input.txt").trim().lines().collect::<Vec<_>>();

    // set X Y sets register X to the value of Y.
    // sub X Y decreases register X by the value of Y.
    // mul X Y sets register X to the result of multiplying the value contained in register X by the value of Y.
    // jnz X Y jumps with an offset of the value of Y, but only if the value of X is not zero. (An offset of 2 skips the next instruction, an offset of -1 jumps to the previous instruction, and so on.)
    {
        let mut registers: HashMap<char, isize> = HashMap::new();
        let mut instruction_pointer: usize = 0;
        let mut mul_instruction_invocations: isize = 0;

        while instruction_pointer < instructions.len() {
            let instruction = instructions[instruction_pointer];
            println!("{}: {}", instruction_pointer, instruction);
            let parts = instruction.split(' ').collect::<Vec<_>>();
            let op = parts[0];

            let arg1 = parse_reg_or_value(parts[1]);
            let arg2 = parts.get(2).map(|x| parse_reg_or_value(x));

            match op {
                "set" => apply(&mut registers, &arg1, arg2, |reg, value| *reg = value),
                "add" => apply(&mut registers, &arg1, arg2, |reg, value| *reg += value),
                "sub" => apply(&mut registers, &arg1, arg2, |reg, value| *reg -= value),
                "mul" => {
                    apply(&mut registers, &arg1, arg2, |reg, value| *reg *= value);
                    mul_instruction_invocations += 1;
                }
                "jnz" => {
                    let value = get_value(&mut registers, &arg1);
                    let offset = get_value(
                        &mut registers,
                        &arg2.expect(&format!("Expected 2 args for {}", op)),
                    );
                    if value != 0 {
                        instruction_pointer = (instruction_pointer as isize + offset) as usize;
                        continue;
                    }
                }
                _ => panic!(format!("Unknown op: {}", op)),
            }

            instruction_pointer = instruction_pointer + 1;
        }
        println!("Part 1: {} mul invocations", mul_instruction_invocations);
    }
}

fn apply(
    regs: &mut HashMap<char, isize>,
    arg1: &Argument,
    arg2: Option<Argument>,
    applier: fn(&mut isize, isize),
) {
    let target = get_register(&arg1);
    let arg2_unwrapped: Argument = arg2.expect("Expected 2 args for math op");
    let value = get_value(regs, &arg2_unwrapped);
    let reg = regs.entry(target).or_insert(0);
    applier(reg, value);
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Argument {
    Register(char),
    Value(isize),
}

fn get_register(argument: &Argument) -> char {
    use self::Argument::*;
    match argument {
        &Register(r) => r,
        _ => panic!(format!("Expected {:?} to be register", argument)),
    }
}

fn get_value(registers: &mut HashMap<char, isize>, argument: &Argument) -> isize {
    use self::Argument::*;
    match argument {
        &Register(r) => *registers.entry(r).or_insert(0),
        &Value(v) => v,
    }
}

fn parse_reg_or_value(s: &str) -> Argument {
    match s.parse() {
        Ok(x) => Argument::Value(x),
        _ => Argument::Register(s.chars().last().unwrap()),
    }
}
