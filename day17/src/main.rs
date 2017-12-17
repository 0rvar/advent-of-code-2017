//use std::collections::LinkedList;

fn main() {
    const STEP_SIZE: usize = 377;

    //let mut list: LinkedList<u32> = LinkedList::new();
    let mut memory: Vec<usize> = Vec::new();
    memory.push(0);
    let mut position = 0;
    let mut size = 1;
    
    for i in 1.. {
        position = (position + STEP_SIZE) % size;
        memory.insert(position, i);
        size = size + 1;

        if i == 2017 {
            println!("{}", memory[(position + 1) % size]);
            break;
        }
    }
}
