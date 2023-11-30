fn main() {
    let lines = include_str!("./input.txt").lines().next().unwrap();
    let out = find_packet_marker(lines);
    println!("Part 1 Answer: {:}", out);
}

fn find_packet_marker(data: &str) -> i32 {
    let (first_4, rest) = data.split_at(4);
    let y: Vec<char> = first_4.chars().collect();
    let mut first;
    let mut second = y[1];
    let mut third = y[2];
    let mut fourth = y[3];

    let mut count = 4;
    for c in rest.chars() {
        count += 1;
        (fourth, third, second, first) = (c, fourth, third, second);
        if first != second
            && first != third
            && first != fourth
            && second != third
            && second != fourth
            && third != fourth
        {
            return count;
        }
    }
    0
}

#[test]
fn test_packet_marker() {
    let i = find_packet_marker("bvwbjplbgvbhsrlpgdmjqwftvncz");
    assert_eq!(i, 5);
    let i = find_packet_marker("nppdvjthqldpwncqszvftbrmjlhg");
    assert_eq!(i, 6);
    let i = find_packet_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
    assert_eq!(i, 10);
    let i = find_packet_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
    assert_eq!(i, 11);
}
