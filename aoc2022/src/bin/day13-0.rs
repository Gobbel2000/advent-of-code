use std::fs;
use std::process::exit;
use std::str::Lines;
use std::cmp::Ordering;

use serde_json::Value;

static INPUT: &str = "input/day13.txt";

fn main() {
    let input = fs::read_to_string(INPUT)
        .unwrap_or_else(|e| {
            eprintln!("Could not read input file: {}", e);
            exit(1);
        });
    let packets = PacketIter{ lines: input.lines() };
    let mut sum = 0;
    for (idx, (p1, p2)) in packets.enumerate() {
        match array_cmp(&p1, &p2) {
            Some(Ordering::Less) | Some(Ordering::Equal) =>{
                //println!("Less: {}:\n{}\n{}", idx, p1, p2);
                sum += idx + 1;
            },
            Some(Ordering::Greater) => {
                //println!("Greater: {}\n{}\n{}", idx, p1, p2);
            },
            None => eprintln!("Could not compare: {}\n{}\n{}", idx, p1, p2),
        }
    }
    println!("{}", sum);
}


struct PacketIter<'a> {
    lines: Lines<'a>,
}

impl<'a> Iterator for PacketIter<'a> {
    type Item = (serde_json::Value, serde_json::Value);

    fn next(&mut self) -> Option<Self::Item> {
        let p1 = self.lines.next()?;
        let p2 = self.lines.next()?;
        let _newline = self.lines.next();
        Some((serde_json::from_str(p1).ok()?,
              serde_json::from_str(p2).ok()?))
    }
}

fn array_cmp(v1: &Value, v2: &Value) -> Option<Ordering> {
    match (v1, v2) {
        (Value::Array(a1), Value::Array(a2)) => {
            vec_cmp(a1, a2)
        },
        (Value::Number(n1), Value::Number(n2)) => {
            return n1.as_i64().expect("Invalid number")
                .partial_cmp(&n2.as_i64().expect("Invalid number"));
        },
        (Value::Array(a), Value::Number(_)) => {
            vec_cmp(a, &vec![v2.clone()])
        },
        (Value::Number(_), Value::Array(a)) => {
            vec_cmp(&vec![v1.clone()], a)
        },
        _ => { eprintln!("Wrong type!"); None },
    }
}

fn vec_cmp(v1: &Vec<Value>, v2: &Vec<Value>) -> Option<Ordering> {
    for (inner1, inner2) in v1.iter().zip(v2) {
        let cmp = array_cmp(inner1, inner2);
        if cmp != Some(Ordering::Equal) {
            //println!("{}   {} => {:?}", inner1, inner2, cmp);
            return cmp;
        }
        if cmp == None {
            eprintln!("Could not compare array elements");
        }
    }
    // If the zipped arrays are equal, compare their lengths
    return v1.len().partial_cmp(&v2.len());
}
