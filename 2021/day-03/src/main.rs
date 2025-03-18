fn main_() {
    let numbers = include_str!("./input.txt")
            .lines()
            .map(|n| n.parse().unwrap())
            .collect::<Vec<u16>>();
    let mut count = 0;
    for window in numbers.windows(2) {
        if window[0] < window[1] {
            count += 1
        }
    }
    println!(
        "{:#?}", count
    );
}

fn main() {
    let numbers = include_str!("./input.txt")
            .lines()
            .map(|n| n.parse().unwrap())
            .collect::<Vec<u16>>();
    let mut prev_sum = 0;
    let mut count = 0;
    for window in numbers.windows(3) {
        let window_sum = window.iter().sum();
        if prev_sum != 0 {
            if prev_sum < window_sum {
                count += 1;
            }
        }
        prev_sum = window_sum;
    }
    println!(
        "{:#?}", count
    );
}

