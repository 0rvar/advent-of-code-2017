fn main() {
    const STEP_SIZE: usize = 377;

    {
        let mut memory: Vec<usize> = Vec::new();
        memory.push(0);
        let mut position = 0;
        let mut size = 1;

        for i in 1.. {
            position = (position + STEP_SIZE) % size;
            memory.insert(position + 1, i);
            position = position + 1;
            size = size + 1;

            if i == 2017 {
                println!("memory[pos]:   {}", memory[(position + 0) % size]);
                println!("memory[pos+1]: {}", memory[(position + 1) % size]);
                break;
            }
        }
    }
    
    {
        let mut zero_position = 0;
        let mut value_after_zero = 0;
        let mut position = 0;
        let mut size = 1;
        for i in 1.. {
            position = (position + STEP_SIZE) % size;
            if position == zero_position {
                value_after_zero = i;
            } else if position < zero_position {
                zero_position += 1;
            }
            position = position + 1;
            size = size + 1;

            if i == 50_000_000 {
                println!("value_after_zero: {}", value_after_zero);
                break;
            }
        }
    }
}
