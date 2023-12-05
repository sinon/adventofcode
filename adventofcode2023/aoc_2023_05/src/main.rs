use std::ops::Range;

#[derive(Debug, PartialEq)]
struct PlantingRange {
    src: Range<i64>,
    dst: Range<i64>,
}

impl PlantingRange {
    fn from_vec(range_mapping: Vec<i64>) -> PlantingRange {
        let dst_initial = range_mapping[0];
        let src_initial = range_mapping[1];
        let r_length = range_mapping[2];
        PlantingRange {
            src: Range {
                start: src_initial,
                end: src_initial + r_length,
            },
            dst: Range {
                start: dst_initial,
                end: dst_initial + r_length,
            },
        }
    }
    fn next_location(&self, mut range_test: i64) -> Option<i64> {
        if self.src.contains(&range_test) {
            range_test += self.dst.start - self.src.start;
            return Some(range_test);
        } else {
            return None;
        }
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let answer_1 = part_1(input);
    println!("Part 1: {}", answer_1);
    let answer_2 = part_2(input);
    println!("Part 2: {}", answer_2);
}

fn part_2(input: &str) -> i64 {
    let mut input = input.split("\n\n");
    let seeds: Vec<i64> = input
        .next()
        .unwrap()
        .replace("seeds: ", "")
        .split(' ')
        .map(|i| i.parse().unwrap())
        .collect();
    println!("{:?}", seeds);
    let mut seed_ranges = vec![];
    for ch in seeds.chunks(2) {
        seed_ranges.push(Range {
            start: ch[0],
            end: ch[0] + ch[1],
        });
        println!("{:?}", ch);
    }
    let maps: Vec<_> = input
        .map(|map| {
            let mut lines = map.lines();
            let name = lines.next().unwrap();
            let mapping = lines
                .map(|ln| {
                    PlantingRange::from_vec(
                        ln.split(' ')
                            .map(|n| n.parse::<i64>().unwrap())
                            .collect::<Vec<_>>(),
                    )
                })
                .collect::<Vec<PlantingRange>>();
            (name, mapping)
        })
        .collect();
    let mut min_route = i64::MAX;
    seed_ranges.into_iter().for_each(|seed_range| {
        seed_range.into_iter().for_each(|s| {
            let mut seed_to_test = s;
            maps.iter().for_each(|(_, ranges)| {
                for planting_range in ranges {
                    if let Some(i) = planting_range.next_location(seed_to_test) {
                        seed_to_test = i;
                        break;
                    }
                }
            });
            min_route = min_route.min(seed_to_test);
        })
    });
    min_route
}

fn part_1(input: &str) -> i64 {
    let mut input = input.split("\n\n");
    let seeds: Vec<i64> = input
        .next()
        .unwrap()
        .replace("seeds: ", "")
        .split(' ')
        .map(|i| i.parse().unwrap())
        .collect();

    let maps: Vec<_> = input
        .map(|map| {
            let mut lines = map.lines();
            let name = lines.next().unwrap();
            let mapping = lines
                .map(|ln| {
                    PlantingRange::from_vec(
                        ln.split(' ')
                            .map(|n| n.parse::<i64>().unwrap())
                            .collect::<Vec<_>>(),
                    )
                })
                .collect::<Vec<PlantingRange>>();
            (name, mapping)
        })
        .collect();
    let mut min_route = i64::MAX;
    seeds.into_iter().for_each(|s| {
        let mut seed_to_test = s;
        maps.iter().for_each(|(_, ranges)| {
            for planting_range in ranges {
                if let Some(i) = planting_range.next_location(seed_to_test) {
                    seed_to_test = i;
                    break;
                }
            }
        });
        min_route = min_route.min(seed_to_test);
    });
    min_route
}

#[test]
fn test_sample_part_1() {
    let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    assert_eq!(part_1(input), 35);
}

#[test]
fn test_sample_part_2() {
    let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    assert_eq!(part_2(input), 46);
}
