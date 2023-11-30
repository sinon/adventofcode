fn main() {
    let lines = include_str!("./input.txt").lines();
    let mut current_elf = 0;
    let mut totals = Vec::new();
    for line in lines.into_iter() {
        if line.is_empty() {
            totals.push(current_elf);
            current_elf = 0;
        } else {
            current_elf += line.parse::<i32>().unwrap();
        }
    }
    totals.sort();

    println!(
        "Highest 3 is: {}",
        totals[totals.len() - 1] + totals[totals.len() - 2] + totals[totals.len() - 3]
    );
}
