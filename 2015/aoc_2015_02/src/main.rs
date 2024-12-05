struct PresentBox {
    length: i32,
    width: i32,
    height: i32,
}

impl PresentBox {
    fn coverage(&self) -> i32 {
        (2 * self.length * self.width)
            + (2 * self.width * self.height)
            + (2 * self.height * self.length)
            + self.shortest()
    }

    fn shortest(&self) -> i32 {
        *vec![
            (self.length * self.width),
            (self.width * self.height),
            (self.height * self.length),
        ]
        .iter()
        .min()
        .unwrap()
    }
    fn from_ln(ln: &str) -> Self {
        let mut values = ln.splitn(3, "x");
        let length = values.next().unwrap().parse().unwrap();
        let width = values.next().unwrap().parse().unwrap();
        let height = values.next().unwrap().parse().unwrap();
        PresentBox {
            length,
            width,
            height,
        }
    }
    fn ribbon_length(&self) -> i32 {
        let mut x = [self.length, self.width, self.height];
        x.sort();
        (2 * x[0] + 2 * x[1]) + (x.iter().product::<i32>())
    }
}

fn main() {
    let input = include_str!("./input.txt").lines();
    let boxes = input.map(|ln| PresentBox::from_ln(ln));
    let p1 = boxes.fold(0, |acc, e| acc + e.coverage());
    dbg!(p1);
    let input = include_str!("./input.txt").lines();
    let boxes = input.map(|ln| PresentBox::from_ln(ln));
    let p2 = boxes.fold(0, |acc, e| acc + e.ribbon_length());
    dbg!(p2);
}

#[test]
fn test_p1() {
    assert_eq!(PresentBox::from_ln("2x3x4").coverage(), 58);
    assert_eq!(PresentBox::from_ln("1x1x10").coverage(), 43);
}

#[test]
fn test_p2() {
    assert_eq!(PresentBox::from_ln("2x3x4").ribbon_length(), 34);
    assert_eq!(PresentBox::from_ln("1x1x10").ribbon_length(), 14);
}
