use std::fs;
use std::process::exit;
use std::cmp::Ordering;

use serde_json::Value;

static INPUT: &str = "input/day13.txt";

fn main() {
    let input = fs::read_to_string(INPUT)
        .unwrap_or_else(|e| {
            eprintln!("Could not read input file: {}", e);
            exit(1);
        });
    let mut packets: Vec<Value> = input.lines()
        .filter(|l| l.len() > 0)
        .map(|l| serde_json::from_str(l).expect("Malformed file"))
        .collect();
    let divider1 = serde_json::to_value(vec![vec![2]]).unwrap();
    let divider2 = serde_json::to_value(vec![vec![6]]).unwrap();
    packets.push(divider1.clone());
    packets.push(divider2.clone());

    packets.sort_by(|v1, v2| array_cmp(v1, v2).expect("Unable to sort"));
    let pos1 = packets.iter().position(|e|
        array_cmp(e, &divider1) == Some(Ordering::Equal)).unwrap();
    let pos2 = packets.iter().position(|e|
        array_cmp(e, &divider2) == Some(Ordering::Equal)).unwrap();
    println!("{}", (pos1 + 1) * (pos2 + 1));
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
