#[macro_use]
extern crate nom;
use nom::{digit, anychar};
use std::str::from_utf8;

#[derive(Clone, Debug, PartialEq, Eq)]
enum DanceMove {
    Spin(u64),
    Exchange(u64, u64),
    Partner(char, char)
}

fn main() {
    let mut programs: Vec<char> = "abcdefghijklmnop".chars().collect();
    let input = include_str!("input.txt").trim();
    let moves = parse_moves(input);

    for m in &moves {
        match m {
            &DanceMove::Spin(s) => apply_spin(&mut programs, s as usize),
            &DanceMove::Exchange(a, b) => apply_exchange(&mut programs, a as usize, b as usize),
            &DanceMove::Partner(a, b) => apply_partner(&mut programs, a, b)
        };
    }
    
    let s: String = programs.iter().cloned().collect();
    println!("{:?}", s);

    for i in 0..1_000_000_000 {
        for m in &moves {
            match m {
                &DanceMove::Spin(s) => apply_spin(&mut programs, s as usize),
                &DanceMove::Exchange(a, b) => apply_exchange(&mut programs, a as usize, b as usize),
                &DanceMove::Partner(a, b) => apply_partner(&mut programs, a, b)
            };
        }
        if i % 10_000 == 0 {
            println!("{}%", i as f64 * 100.0 / 1_000_000_000.0);
        }
    }

    let s: String = programs.iter().cloned().collect();
    println!("{:?}", s);
}

fn apply_partner(programs: &mut [char], a: char, b: char) {
    let a_index = programs.iter().position(|&x| x == a).unwrap();
    let b_index = programs.iter().position(|&x| x == b).unwrap();
    let a_value = programs[a_index];
    let b_value = programs[b_index];

    programs[a_index] = b_value;
    programs[b_index] = a_value;
}

#[test]
fn test_apply_partner() {
    let p = |s: &str| s.chars().collect::<Vec<_>>();
    {
        let mut ps = p("eabdc");
        apply_partner(&mut ps, 'e', 'b');
        assert_eq!(ps, p("baedc"));
    }
    {
        let mut ps = p("abcde");
        apply_partner(&mut ps, 'c', 'a');
        assert_eq!(ps, p("cbade"));
    }
    {
        let mut ps = p("abcde");
        apply_partner(&mut ps, 'a', 'b');
        assert_eq!(ps, p("bacde"));
    }
}

fn apply_exchange(programs: &mut [char], a: usize, b: usize) {
    let a_value = programs[a];
    let b_value = programs[b];

    programs[a] = b_value;
    programs[b] = a_value;
}

#[test]
fn test_apply_exchange() {
    let p = |s: &str| s.chars().collect::<Vec<_>>();
    {
        let mut ps = p("eabcd");
        apply_exchange(&mut ps, 3, 4);
        assert_eq!(ps, p("eabdc"));
    }
    {
        let mut ps = p("abcde");
        apply_exchange(&mut ps, 0, 4);
        assert_eq!(ps, p("ebcda"));
    }
    {
        let mut ps = p("abcde");
        apply_exchange(&mut ps, 1, 0);
        assert_eq!(ps, p("bacde"));
    }
}

fn apply_spin(programs: &mut [char], index: usize) {
    let split_index = programs.len() - index;

    let tail_copy = programs[split_index..].to_vec();

    for i in (0 .. programs.len() - index).rev() {
        programs[i + index] = programs[i];
    }
    for i in 0 .. index {
        programs[i] = tail_copy[i];
    }
}

#[test]
fn test_apply_spin() {
    let p = |s: &str| s.chars().collect::<Vec<_>>();
    {
        let mut ps = p("abcde");
        apply_spin(&mut ps, 1);
        assert_eq!(ps, p("eabcd"));
    }
    {
        let mut ps = p("abcde");
        apply_spin(&mut ps, 2);
        assert_eq!(ps, p("deabc"));
    }
    {
        let mut ps = p("abcde");
        apply_spin(&mut ps, 3);
        assert_eq!(ps, p("cdeab"));
    }
    {
        let mut ps = p("abcde");
        apply_spin(&mut ps, 4);
        assert_eq!(ps, p("bcdea"));
    }
    {
        let mut ps = p("abcde");
        apply_spin(&mut ps, 5);
        assert_eq!(ps, p("abcde"));
    }
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




