use std::sync::LazyLock;

use regex::Regex;

#[derive(Debug)]
struct Machine {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

impl Machine {
    fn tokens_100(&self) -> i64 {
        for b in 0..=100 {
            for a in 0..=100 {
                let pos = (self.a.0 * a + self.b.0 * b, self.a.1 * a + self.b.1 * b);
                if pos == self.prize {
                    return b + 3 * a;
                }
            }
        }
        0
    }

    fn tokens_inv(&self) -> i64 {
        // If [ab] is the matrix containing our two button vectors: [ a.0 b.0 ]
        //                                                          [ a.1 b.1 ]
        // then prize = [ab] * x, where x holds the number of required button presses
        // for a and b, (na, nb).
        // By inverting [ab] we get
        //
        // x = [ab]⁻¹ * prize
        let det = (self.a.0 * self.b.1) - (self.a.1 * self.b.0);
        if det == 0 {
            panic!("Irregular matrix");
        }
        let det = det as f64;
        // The matrix [ a b ] is the inverse of [ a.0 b.0 ] .
        //            [ c d ]                   [ a.1 b.1 ]
        let a = self.b.1 as f64 / det;
        let b = -self.b.0 as f64 / det;
        let c = -self.a.1 as f64 / det;
        let d = self.a.0 as f64 / det;
        // Multiply [ab] * prize to get the result
        let na = self.prize.0 as f64 * a + self.prize.1 as f64 * b;
        let nb = self.prize.0 as f64 * c + self.prize.1 as f64 * d;

        // Only integer solutions are valid, verify rounded results:
        let ina = na.round() as i64;
        let inb = nb.round() as i64;
        let pos = (
            self.a.0 * ina + self.b.0 * inb,
            self.a.1 * ina + self.b.1 * inb,
        );
        if pos == self.prize {
            inb + 3 * ina
        } else {
            0
        }
    }

    fn translate(&self, tr: i64) -> Self {
        let prize = (self.prize.0 + tr, self.prize.1 + tr);
        Machine { prize, ..*self }
    }
}

impl From<&str> for Machine {
    fn from(s: &str) -> Self {
        static RE: LazyLock<(Regex, Regex)> = LazyLock::new(|| {
            (
                Regex::new(r"Button [AB]: X\+(\d+), Y\+(\d+)").unwrap(),
                Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap(),
            )
        });
        let (re_btn, re_prize) = &*RE;
        let mut caps = re_btn.captures_iter(s);
        let (_, [a0, a1]) = caps.next().unwrap().extract();
        let a = (a0.parse().unwrap(), a1.parse().unwrap());
        let (_, [b0, b1]) = caps.next().unwrap().extract();
        let b = (b0.parse().unwrap(), b1.parse().unwrap());
        let (_, [p0, p1]) = re_prize.captures(s).unwrap().extract();
        let prize = (p0.parse().unwrap(), p1.parse().unwrap());
        Machine { a, b, prize }
    }
}

fn parse(input: String) -> Vec<Machine> {
    input.split("\n\n").map(Into::into).collect()
}

fn part1(input: String) {
    let machines = parse(input);
    let sum = machines.iter().map(|m| m.tokens_100()).sum::<i64>();
    println!("{sum}");
}

const TRANSLATION: i64 = 10000000000000;

fn part2(input: String) {
    let machines = parse(input);
    let sum = machines
        .iter()
        .map(|m| m.translate(TRANSLATION).tokens_inv())
        .sum::<i64>();
    println!("{sum}");
}

util::aoc_main!();
