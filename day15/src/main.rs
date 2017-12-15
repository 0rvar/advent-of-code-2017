fn main() {
    let starting_values = (679usize, 771usize);
    let limit = 40_000_000;
    
    let (factor_a, factor_b) = (16807usize, 48271usize);
    let divisor = 2147483647;

    let (mut a, mut b) = starting_values;
    let mut num_matches = 0;
    for _ in 0..limit {
        a = (a * factor_a) % divisor;
        b = (b * factor_b) % divisor;
        
        if low16_equal(a, b) {
            num_matches += 1;
        }
    }

    println!("Num matches: {}", num_matches);

    let (mut a, mut b) = starting_values;
    let mut num_matches = 0;
    let limit = 5_000_000;
    for _ in 0..limit {
        loop {
            a = (a * factor_a) % divisor;
            if a % 4 == 0 { break }
        }
        loop {
            b = (b * factor_b) % divisor;
            if b % 8 == 0 { break }
        }
        
        if low16_equal(a, b) {
            num_matches += 1;
        }
    }
    println!("Num matches: {}", num_matches);

}

fn low16_equal(left: usize, right: usize) -> bool {
    left as u16 == right as u16
}

#[test]
fn test_low16_equal() {
    assert!(!low16_equal(
        0b01100111111110000001011011000111,
        0b00001000001101111100110000000111
    ));
    assert!(low16_equal(
        0b00001110101000101110001101001010,
        0b01010101010100101110001101001010
    ));
}
