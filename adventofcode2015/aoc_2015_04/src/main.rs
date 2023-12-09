fn main() {
    let input = "iwrupvqb";
    let p1_answer = p1(input);
    dbg!(p1_answer);
    let p2_answer = p2(input);
    dbg!(p2_answer);
}

fn p1(input: &str) -> i64 {
    for i in 1..1000000000 {
        let digest = md5::compute(format!("{}{}", input, i));
        let s_str = format!("{:x}", digest);
        if s_str.starts_with("00000") {
            return i;
        }
    }
    0
}
fn p2(input: &str) -> i64 {
    for i in 346385..1000000000 {
        let digest = md5::compute(format!("{}{}", input, i));
        let s_str = format!("{:x}", digest);
        if s_str.starts_with("000000") {
            return i;
        }
    }
    0
}

#[test]
fn test_p1() {
    assert_eq!(p1("abcdef"), 609043);
    assert_eq!(p1("pqrstuv"), 1048970);
}
