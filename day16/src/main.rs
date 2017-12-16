#[macro_use]
extern crate nom;
use nom::{digit, anychar};
use std::str::from_utf8;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
enum DanceMove {
    Spin(u64),
    Exchange(u64, u64),
    Partner(char, char)
}

fn main() {
    let original_programs: Vec<char> = "abcdefghijklmnop".chars().collect();
    let input = include_str!("input.txt").trim();
    let moves = parse_moves(input);

    let mut part1_programs = original_programs.clone();
    for m in &moves {
        part1_programs = match m {
            &DanceMove::Spin(s) => apply_spin(&part1_programs, s as usize),
            &DanceMove::Exchange(a, b) => apply_exchange(&part1_programs, a as usize, b as usize),
            &DanceMove::Partner(a, b) => apply_partner(&part1_programs, a, b)
        };
    }

    println!("{:?}", part1_programs.iter().cloned().collect::<String>());

    let mut programs = original_programs.clone();
    let mut cycle_map: HashMap<String, usize> = HashMap::new();
    let mut period = 0;
    for i in 0.. {
        let hash = programs_to_string(&programs);
        if cycle_map.contains_key(&hash) {
            println!("Found cycle at: {}: {}", i, cycle_map.get(&hash).unwrap());
            period = i;
            break;
        } else {
            cycle_map.insert(hash, i);
        }

        for m in &moves {
            programs = match m {
                &DanceMove::Spin(s) => apply_spin(&programs, s as usize),
                &DanceMove::Exchange(a, b) => apply_exchange(&programs, a as usize, b as usize),
                &DanceMove::Partner(a, b) => apply_partner(&programs, a, b)
            };
        }
    }

    for _ in 0 .. (1_000_000_000 % period) {
        for m in &moves {
            programs = match m {
                &DanceMove::Spin(s) => apply_spin(&programs, s as usize),
                &DanceMove::Exchange(a, b) => apply_exchange(&programs, a as usize, b as usize),
                &DanceMove::Partner(a, b) => apply_partner(&programs, a, b)
            };
        }
    }

    println!("{:?}", programs.iter().cloned().collect::<String>());
}

fn programs_to_string(programs: &[char]) -> String {
    programs.iter().cloned().collect::<String>()
}

fn apply_partner(programs: &[char], a: char, b: char) -> Vec<char> {
    let a_index = programs.iter().position(|&x| x == a).unwrap();
    let b_index = programs.iter().position(|&x| x == b).unwrap();
    let left_index = if a_index < b_index { a_index } else { b_index };
    let right_index = if a_index > b_index { a_index } else { b_index };
    let left_value = programs[left_index];
    let right_value = programs[right_index];

    let (mut left, mut middle, mut right) = (
        programs[0 .. left_index].to_vec(),
        programs[left_index + 1 .. right_index].to_vec(),
        programs[right_index + 1 ..].to_vec()
    );
    left.push(right_value);
    left.append(&mut middle);
    left.push(left_value);
    left.append(&mut right);
    left
}

#[test]
fn test_apply_partner() {
    let p = |s: &str| s.chars().collect::<Vec<_>>();
    assert_eq!(apply_partner(&p("eabdc"), 'e', 'b'), p("baedc"));
    assert_eq!(apply_partner(&p("abcde"), 'c', 'a'), p("cbade"));
    assert_eq!(apply_partner(&p("abcde"), 'a', 'b'), p("bacde"));
}

fn apply_exchange(programs: &[char], a: usize, b: usize) -> Vec<char> {
    let left_index = if a < b { a } else { b };
    let right_index = if a > b { a } else { b };
    let left_value = programs[left_index];
    let right_value = programs[right_index];

    let (mut left, mut middle, mut right) = (
        programs[0 .. left_index].to_vec(),
        programs[left_index + 1 .. right_index].to_vec(),
        programs[right_index + 1 ..].to_vec()
    );
    left.push(right_value);
    left.append(&mut middle);
    left.push(left_value);
    left.append(&mut right);
    left
}

#[test]
fn test_apply_exchange() {
    let p = |s: &str| s.chars().collect::<Vec<_>>();
    assert_eq!(apply_exchange(&p("eabcd"), 3, 4), p("eabdc"));
    assert_eq!(apply_exchange(&p("abcde"), 0, 4), p("ebcda"));
    assert_eq!(apply_exchange(&p("abcde"), 1, 0), p("bacde"));
}

fn apply_spin(programs: &[char], index: usize) -> Vec<char> {
    let split_index = programs.len() - index;
    let (mut left, mut right) = (programs[0..split_index].to_vec(), programs[split_index ..].to_vec());

    right.append(&mut left);
    right
}

#[test]
fn test_apply_spin() {
    let p = |s: &str| s.chars().collect::<Vec<_>>();
    assert_eq!(apply_spin(&p("abcde"), 1), p("eabcd"));
    assert_eq!(apply_spin(&p("abcde"), 2), p("deabc"));
    assert_eq!(apply_spin(&p("abcde"), 3), p("cdeab"));
    assert_eq!(apply_spin(&p("abcde"), 4), p("bcdea"));
    assert_eq!(apply_spin(&p("abcde"), 5), p("abcde"));
}



fn parse_moves(input: &str) -> Vec<DanceMove> {
    input.split(",").map(|w| parse_dance_move(w.as_bytes()).to_result().unwrap()).collect::<Vec<_>>()
}

#[test]
fn test_parse_moves() {
    assert_eq!(parse_moves("s1,x3/4,pe/b"), vec![
        DanceMove::Spin(1),
        DanceMove::Exchange(3, 4),
        DanceMove::Partner('e', 'b')
    ]);
}


named!(parse_u64_utf8 <&[u8], u64>, do_parse!(
        bytes: digit >>
        (from_utf8(bytes).unwrap().parse().unwrap())
));

named!(parse_spin <&[u8], u64>, do_parse!(
        tag!(b"s") >>
        n: parse_u64_utf8 >>
        (n)
));

named!(parse_exchange <&[u8], (u64, u64)>, do_parse!(
        tag!(b"x") >>
        n: parse_u64_utf8 >>
        tag!(b"/") >>
        m: parse_u64_utf8 >>
        ((n, m))
));

named!(parse_partner <&[u8], (char, char)>, do_parse!(
        tag!(b"p") >>
        n: anychar >>
        tag!(b"/") >>
        m: anychar >>
        ((n, m))
));

named!(parse_dance_move<&[u8], DanceMove>, alt!(
        do_parse!(s: parse_spin >> (DanceMove::Spin(s))) |
        do_parse!(e: parse_exchange >> (DanceMove::Exchange(e.0, e.1))) |
        do_parse!(p: parse_partner >> (DanceMove::Partner(p.0, p.1)))
));

#[test]
fn test_parse_move() {
    assert_eq!(parse_dance_move(b"s3").to_result().unwrap(), DanceMove::Spin(3));
    assert_eq!(parse_dance_move(b"x3/4").to_result().unwrap(), DanceMove::Exchange(3, 4));
    assert_eq!(parse_dance_move(b"pA/X").to_result().unwrap(), DanceMove::Partner('A', 'X'));
}




