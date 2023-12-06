fn main() {
    let input = vec![(47, 400), (98, 1213), (66, 1011), (98, 1540)];
    let part_1 = p1(input);
    dbg!(part_1);
    let part_2 = p2(47986698, 400121310111540);
    dbg!(part_2);
}

fn p2(race_duration: i64, distance_to_beat: i64) -> i64 {
    let mut x = durations(race_duration, distance_to_beat);
    x.dedup();
    x.len() as i64
}

fn p1(input: Vec<(i64, i64)>) -> i64 {
    let mut total: i64 = 1;
    for (race_duration, distance_to_beat) in input {
        total *= durations(race_duration, distance_to_beat).len() as i64;
    }
    total
}

fn durations(race_duration: i64, distance_to_beat: i64) -> Vec<i64> {
    let mut potentials = vec![];
    for i in 1..race_duration {
        let rem = race_duration - i;
        let distance_covered = rem * i;

        if distance_covered > distance_to_beat {
            potentials.push(i);
        }
    }
    potentials
}

#[test]
fn test_durations() {
    assert_eq!(durations(7, 9).len(), 4);
    assert_eq!(durations(15, 40).len(), 8);
    assert_eq!(durations(30, 200).len(), 9);
}

#[test]
fn test_p1() {
    let input = vec![(7, 9), (15, 40), (30, 200)];
    assert_eq!(p1(input), 288);
}

#[test]
fn test_p2() {
    assert_eq!(p2(71530, 940200), 71503);
}
