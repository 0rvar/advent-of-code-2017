fn main() {
    let input = include_str!("input.txt");
    let pass_phrases: Vec<Vec<&str>> = input
        .lines()
        .map(parse_passphrase)
        .collect();

    let valid: usize = pass_phrases.iter().filter(|x| validate(&x)).count();
    let secure_valid: usize = pass_phrases.iter().filter(|x| validate_secure(&x)).count();

    println!("Total: {} passphrases", pass_phrases.len());
    println!("Valid: {} passphrases", valid);
    println!("Secure valid: {} passphrases", secure_valid);
    
}


fn parse_passphrase(line: &str) -> Vec<&str> {
    line
        .split(' ')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect()
}

fn validate(passphrase: &[&str]) -> bool {
    use std::collections::HashSet;
    let mut uniq = HashSet::new();
    passphrase.into_iter().all(move |x| uniq.insert(x))
}

#[test]
fn it_validates_some_passwords() {
    assert_eq!(validate(&["aa", "bb"]), true);
    assert_eq!(validate(&["aa", "aa"]), false);
}

fn validate_secure(passphrase: &[&str]) -> bool {
    use std::collections::HashSet;
    use std::iter::FromIterator;
    let mut uniq = HashSet::new();
    passphrase.into_iter().all(move |x| {
        let mut chars: Vec<char> = x.chars().collect();
        chars.sort_by(|a, b| b.cmp(a));
        let s = String::from_iter(chars);
        uniq.insert(s)
    })
}

#[test]
fn it_securely_validates_some_passwords() {
    assert_eq!(validate_secure(&["aab", "bba"]), true);
    assert_eq!(validate_secure(&["aab", "aba"]), false);
}