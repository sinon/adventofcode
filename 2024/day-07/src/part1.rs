use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar_inline = r#"
result   = { (ASCII_DIGIT)+ }
operands = { (ASCII_DIGIT | "." | "-")+ }
record   = { result ~ ":" ~ (" " ~ operands)* }
file     = { SOI ~ (record ~ ("\r\n" | "\n" | EOI))* ~ EOI }
"#]
struct ParseFormulas;

#[derive(Debug)]
struct Formula {
    result: Option<i64>,
    operands: Vec<i64>,
}

fn generate_combinations<T: Clone>(input: &[T], slots: usize) -> Vec<Vec<T>> {
    if slots == 0 {
        return Vec::new();
    }

    let mut result: Vec<Vec<T>> = input.iter().map(|x| vec![x.clone()]).collect();

    for _ in 1..slots {
        result = result
            .into_iter()
            .flat_map(|current_combo| {
                input.iter().map(move |x| {
                    let mut new_combo = current_combo.clone();
                    new_combo.push(x.clone());
                    new_combo
                })
            })
            .collect();
    }

    result
}

impl Formula {
    fn can_balance(&self) -> i64 {
        if self.operands.is_empty() {
            return 0;
        }
        let combinations = generate_combinations(
            &[Operator::Multiply, Operator::Add],
            self.operands.len() - 1,
        );
        for combination in combinations {
            let mut a = self.operands[0];
            let mut result = 0;
            // [81, 40, 27]
            for (idx, op) in combination.iter().enumerate() {
                let b = self.operands[idx + 1];
                result = apply_operator(op, a, b);
                a = result;
            }
            if let Some(r) = self.result {
                if r == result {
                    return result;
                }
            }
        }
        0
    }
}

fn apply_operator(op: &Operator, a: i64, b: i64) -> i64 {
    match *op {
        Operator::Add => a + b,
        Operator::Multiply => a * b,
    }
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Multiply,
    Add,
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<i64> {
    let file = ParseFormulas::parse(Rule::file, _input)
        .expect("Unsuccessful parse")
        .next()
        .unwrap();
    let mut formulas: Vec<Formula> = Vec::new();
    for record in file.into_inner() {
        let mut operands: Vec<i64> = Vec::new();
        let mut result: Option<i64> = None;
        for element in record.into_inner() {
            match element.as_rule() {
                Rule::result => {
                    result = Some(element.as_str().parse::<i64>().unwrap());
                }
                Rule::operands => {
                    let operand = element.as_str().parse::<i64>().unwrap();
                    operands.push(operand);
                }
                Rule::EOI => {
                    println!("EOF");
                }
                _ => {
                    print!("{:?}", element.into_inner())
                }
            }
        }

        formulas.push(Formula { result, operands });
    }
    let result = formulas.iter().map(|f| f.can_balance()).sum();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!(3749, process(input)?);
        Ok(())
    }
}
