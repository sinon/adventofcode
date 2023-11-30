use std::collections::VecDeque;

fn main() {
    let lines = include_str!("./input.txt").lines().next().unwrap();
    let out = find_packet_marker(lines, 4);
    println!("Part 1 Answer: {:}", out);
    let out = find_packet_marker(lines, 14);
    println!("Part 2 Answer: {:}", out);
}

fn is_char_array_unique(ch_arr: Vec<char>) -> bool {
    let mut tmp = ch_arr.clone();
    tmp.sort();
    tmp.dedup();
    ch_arr.len() == tmp.len()
}

fn find_packet_marker(data: &str, size: usize) -> usize {
    let (first_n, rest) = data.split_at(size);
    let mut buf: VecDeque<char> = VecDeque::new();
    for i in first_n.chars() {
        buf.push_back(i);
    }

    let mut count = size;
    for c in rest.chars() {
        count += 1;
        buf.push_back(c);
        buf.pop_front();
        if is_char_array_unique(buf.clone().into()) {
            return count;
        }
    }
    0
}

#[test]
fn test_packet_marker_4() {
    let i = find_packet_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4);
    assert_eq!(i, 5);
    let i = find_packet_marker("nppdvjthqldpwncqszvftbrmjlhg", 4);
    assert_eq!(i, 6);
    let i = find_packet_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4);
    assert_eq!(i, 10);
    let i = find_packet_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4);
    assert_eq!(i, 11);
}

#[test]
fn test_packet_marker_14() {
    let i = find_packet_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14);
    assert_eq!(i, 19);
    let i = find_packet_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 14);
    assert_eq!(i, 23);
    let i = find_packet_marker("nppdvjthqldpwncqszvftbrmjlhg", 14);
    assert_eq!(i, 23);
    let i = find_packet_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14);
    assert_eq!(i, 29);
    let i = find_packet_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14);
    assert_eq!(i, 26);
}
