fn main() {
    let input = include_str!("input.txt");
    let source: Vec<i32> = input
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();

    {
        let mut instructions = source.to_vec();
        let mut pos: i32 = 0;
        let mut moves: i32 = 0;
        while pos >= 0 && pos < instructions.len() as i32 {
            let jump = instructions[pos as usize];
            instructions[pos as usize] = jump + 1;
            pos = pos + jump;
            moves = moves + 1;
        }
        println!("Moves: {}", moves);
    }

    {
        let mut instructions = source.to_vec();
        let mut pos: i32 = 0;
        let mut moves: i32 = 0;
        while pos >= 0 && pos < instructions.len() as i32 {
            let jump = instructions[pos as usize];
            if jump >= 3 {
                instructions[pos as usize] = jump - 1;
            } else {
                instructions[pos as usize] = jump + 1;
            }
            pos = pos + jump;
            moves = moves + 1;
        }
        println!("Moves: {}", moves);
    }
}