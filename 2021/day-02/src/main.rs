fn main_() {
    let (forward, depth) = include_str!("./input.txt")
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .fold((0, 0), | (forward, depth), (action, value) | {
            match (action, value.parse::<i32>().unwrap()) {
                ("forward", v) => (forward + v, depth),
                ("down", v) => (forward, depth + v),
                ("up", v) => (forward, depth - v),
                _ => panic!(),
            }
        });
    println!("{}", forward);
    println!("{}", depth);
    println!("{}", forward * depth);
}

fn main() {
    let (forward, depth, aim) = include_str!("./input.txt")
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .fold((0, 0, 0), | (forward, depth, aim), (action, value) | {
            match (action, value.parse::<i32>().unwrap()) {
                ("forward", v) => (forward + v, depth + (aim * v), aim),
                ("down", v) => (forward, depth, aim + v),
                ("up", v) => (forward, depth, aim - v),
                _ => panic!(),
            }
        });
    println!("{}", forward);
    println!("{}", depth);
    println!("{}", aim);
    println!("{}", forward * depth);
}

