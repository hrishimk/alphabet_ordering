use std::cmp::Ordering;
use std::io;

fn main() {
    let mut alph = String::new();
    io::stdin().read_line(&mut alph).unwrap();
    let alph: String = alph
        .trim()
        .chars()
        .map(|a| vec![a.to_ascii_uppercase(), a.to_ascii_lowercase()])
        .flatten()
        .collect();

    let mut chars = String::new();
    io::stdin().read_line(&mut chars).unwrap();
    let mut chars: Vec<char> = chars
        .trim()
        .chars()
        .collect();
    chars.sort_by(|a, b| c_cmp(&alph, *a, *b));

    println!("{}", chars.iter().collect::<String>());
}

fn c_cmp(aset: &str, c1: char, c2: char) -> Ordering {
    let pos1 = aset.find(c1).unwrap();
    let pos2 = aset.find(c2).unwrap();

    if pos1 < pos2 {
        Ordering::Less
    } else if pos1 > pos2 {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}
