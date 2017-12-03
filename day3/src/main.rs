// 37  36  35  34  33  32  31
// 38  17  16  15  14  13  30
// 39  18   5   4   3  12  29
// 40  19   6   1   2  11  28
// 41  20   7   8   9  10  27
// 42  21  22  23  24  25  26
// 43  44  45  46  47  48  49 50

// 65  64  63  62  61  60  59  58  57
// 66  37  36  35  34  33  32  31  56
// 67  38  17  16  15  14  13  30  55
// 68  39  18   5   4   3  12  29  54
// 69  40  19   6   1   2  11  28  53
// 70  41  20   7   8   9  10  27  52
// 71  42  21  22  23  24  25  26  51
// 72  43  44  45  46  47  48  49  50
// 73  74  75  76  77  78  79  80  81

// 1^2 = 1
// 3^2 = 9
// 5^2 = 25

// 1 9 25 49
// 0 8 24 48
// 1 8 16 32



// 10  11  12  13  14  15  16  17  18  19  20  21  22  23  24  25
// 4   3   4   5   4   3   4   5   4   3   4   5   4   3   4   5
// --
// 2   1   2   3   2   1   2   3   2   1   2   3   2   1   2   3
// --
// 15  14  13  12  11  10  9   8   7   6   5   4   3   2   1   0
// --
// 4   3   2   1   2   3   4
// 6   5   4   3   2   1   0

//     N  
//  N [N] N
//     N

fn main() {
    let input = 361527;
    let distance = spiral_distance(input);
    println!("Distance for {}: {}", input, distance);
}

fn spiral_distance(number: usize) -> u32 {
    let n: u32 = match (1u32..).find(|x| (
        x % 2 != 0 && x * x >= number as u32
    )) {
        Some(x) => x,
        _ => return 0
    };
    let position = n * n;
    let distance = n - 1;

    println!("=== NUMBER: {}", number);
    println!("n: {}", n);
    println!("distance: {}", distance);
    println!("position: {}", position);

    let mut diagonal_offset = (-(NUMBER as isize - position as isize)) as u32 % distance;
    if diagonal_offset > (distance / 2) {
        diagonal_offset = distance - diagonal_offset;
    }

    println!("diagonal_offset: {}", diagonal_offset);

    return distance - diagonal_offset;
}

#[test]
fn calculate_some_distances() {
    assert_eq!(spiral_distance(9), 2);
    assert_eq!(spiral_distance(25), 4);
    assert_eq!(spiral_distance(49), 6);
    assert_eq!(spiral_distance(81), 8);
    assert_eq!(spiral_distance(73), 8);
    assert_eq!(spiral_distance(8), 1);
    assert_eq!(spiral_distance(10), 3);
    assert_eq!(spiral_distance(11), 2);
    assert_eq!(spiral_distance(12), 3);
    assert_eq!(spiral_distance(23), 2);
    assert_eq!(spiral_distance(50), 7);
    assert_eq!(spiral_distance(68), 5);
    assert_eq!(spiral_distance(1024), 31);
}
