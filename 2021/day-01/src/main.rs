const WIDTH: usize = 12;

fn main() {
    let lines = include_str!("./input.txt").lines();
    let mut index_counts = vec![0; WIDTH];

    for line in lines.into_iter() {
        for (index, char) in line.chars().enumerate() {
            if char == '1' {
                index_counts[index] += 1;
            }
        }
    }
    let mut gamma_reading = String::from("");
    let mut epsilon_reading = String::from("");
    for count in index_counts {
        // Half length of input.txt
        if count >= 500 {
            gamma_reading.push('1');
            epsilon_reading.push('0');
        } else {
            gamma_reading.push('0');
            epsilon_reading.push('1');
        }
    }
    let gamma_b10 = isize::from_str_radix(gamma_reading.as_str(), 2).unwrap();
    let epsilon_b10 = isize::from_str_radix(epsilon_reading.as_str(), 2).unwrap();
    println!("{:?}", gamma_b10 * epsilon_b10);
}
