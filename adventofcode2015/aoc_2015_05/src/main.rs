#![feature(iter_map_windows)]
fn is_nice(ln: &str) -> bool {
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    let mut vowels_found: Vec<char> = Vec::new();
    // contains at least vowels
    let mut vowel_count = 0;
    for c in ln.chars() {
        if VOWELS.contains(&c.to_ascii_lowercase()) {
            if !vowels_found.contains(&c.to_ascii_lowercase()) {
                vowels_found.push(c);
            }
            vowel_count += 1;
        }
    }
    // aa
    let dupe_chars = ln.chars().map_windows(|[x, y]| x == y).any(|c| c);

    // !(ab, cd, pq, xy)
    let bad_pairs = ["ab", "cd", "pq", "xy"];
    let no_bad_strs = bad_pairs
        .iter()
        .map(|x| ln.find(*x).is_some())
        .all(|x| x == false);
    let x: Vec<bool> = bad_pairs.iter().map(|x| ln.find(x).is_some()).collect();
    if vowel_count > 2 && no_bad_strs && dupe_chars {
        return true;
    }
    false
}

fn main() {
    let input = include_str!("./input.txt");
    let mut total_nice = 0;
    for ln in input.lines() {
        if is_nice(ln) {
            total_nice += 1;
        }
    }
    dbg!(total_nice);
}

#[test]
fn test_p1() {
    assert!(is_nice("ugknbfddgicrmopn"));
    assert!(is_nice("aaa"));
    assert!(!is_nice("jchzalrnumimnmhp"));
    assert!(!is_nice("haegwjzuvuyypxyu"));
    assert!(!is_nice("dvszwmarrgswjxmb"));
}
