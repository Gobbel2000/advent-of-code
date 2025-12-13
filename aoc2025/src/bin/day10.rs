use std::collections::VecDeque;

use ndarray::{Array1, Array2, Axis, s};

// Tolerance used for checking if an f64 approximates 0
const EPS: f64 = 0.0001;

struct Machine {
    goal: u16,
    buttons: Vec<u16>,
    joltages: Vec<u16>,
}

fn parse_input(input: &str) -> Vec<Machine> {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split_ascii_whitespace();
            let first = parts.next().unwrap();
            let goal = first[1..first.len() - 1]
                .chars()
                .enumerate()
                .fold(0u16, |acc, (i, c)| acc | (((c == '#') as u16) << i));
            let p2 = parts.clone();
            let mut buttons: Vec<u16> = parts
                .take_while(|p| p.starts_with('('))
                .map(|p| {
                    p[1..p.len() - 1]
                        .split(',')
                        .map(|n| n.parse::<u8>().unwrap())
                        .fold(0, |acc, x| acc + (1 << x))
                })
                .collect();
            let last = p2.last().unwrap();
            let joltages: Vec<u16> = last[1..last.len() - 1]
                .split(',')
                .map(|n| n.parse::<u16>().unwrap())
                .collect();
            // Sort buttons to reduce search space
            buttons.sort_by_key(|b| {
                u16::MAX
                    - *joltages
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| (b & (1 << i)) != 0)
                        .min()
                        .unwrap()
                        .1
            });

            Machine {
                goal,
                buttons,
                joltages,
            }
        })
        .collect()
}

// Based on:
// https://en.wikipedia.org/wiki/Gaussian_elimination#Pseudocode
fn gauss_elim(mut a: Array2<f64>) -> Array2<f64> {
    let mut h = 0;
    let mut k = 0;
    let (m, n) = a.dim();
    while h < m && k < n {
        // Find best pivot
        let i_max = a
            .index_axis(Axis(1), k)
            .iter()
            .enumerate()
            .skip(h)
            .max_by(|a, b| a.1.abs().partial_cmp(&b.1.abs()).unwrap())
            .unwrap()
            .0;
        if a[[i_max, k]] == 0.0 {
            // No pivot, pass to next column
            k += 1;
        } else {
            // Swap rows
            for j in 0..n {
                a.swap([h, j], [i_max, j]);
            }
            for i in (h + 1)..m {
                if a[[i, k]] != 0.0 {
                    let f = a[[i, k]] / a[[h, k]];
                    // Lower part of pivot column will be zeros
                    a[[i, k]] = 0.0;
                    for j in (k + 1)..n {
                        // Subtraction for the remaining row
                        a[[i, j]] -= a[[h, j]] * f;
                    }
                }
            }
            h += 1;
            k += 1;
        }
    }
    a
}

impl Machine {
    fn min_presses(&self) -> u32 {
        let mut queue = VecDeque::from([(0, 0, 0)]);
        while let Some((state, min_idx, presses)) = queue.pop_front() {
            for (i, b) in self.buttons.iter().enumerate().skip(min_idx) {
                let next = state ^ b;
                if next == self.goal {
                    return presses + 1;
                }
                queue.push_back((next, i + 1, presses + 1));
            }
        }
        panic!("No solution found!");
    }

    // Augmented matrix representing the linear equation system
    fn matrix(&self) -> Array2<f64> {
        let mut ab = Array2::zeros((self.joltages.len(), self.buttons.len() + 1));
        for (i, jolt) in self.joltages.iter().enumerate() {
            for (j, b) in self.buttons.iter().enumerate() {
                ab[[i, j]] = ((b >> i) & 1) as f64;
            }
            ab[[i, self.buttons.len()]] = *jolt as f64;
        }
        ab
    }

    fn rec_search(
        &self,
        mat: &Array2<f64>,
        mut assignment: Vec<Option<u16>>,
        row: usize,
        col: usize,
    ) -> Result<Vec<Option<u16>>, Vec<Option<u16>>> {
        let (_m, n) = mat.dim();
        // Coefficient vector
        let a = mat.slice(s![row, ..(n - 1)]);
        // Target value
        let b = mat[[row, n - 1]];
        // Indices of unspecified variables in this row
        let i_missing: Vec<usize> = a
            .iter()
            .take(col + 1)
            .enumerate()
            .filter(|(i, e)| e.abs() > EPS && assignment[*i].is_none())
            .map(|(i, _)| i)
            .collect();
        let known: f64 = a
            .iter()
            .zip(&assignment)
            .map(|(a, x)| *a * x.unwrap_or(0) as f64)
            .sum();
        let missing = b - known;

        let n_missing = i_missing.len();
        if n_missing == 0 {
            // All variables for this row are present. Verify, then move to next row.
            if (known - b).abs() > EPS {
                // Equation does not add up to target
                return Err(assignment);
            }
            if row == 0 {
                return Ok(assignment);
            } else {
                return self.rec_search(mat, assignment, row - 1, a.len() - 1);
            }
        } else if n_missing == 1 {
            // Exactly one variably for this row is unspecified, we can calculate it
            let val_f = missing / a[i_missing[0]];
            if val_f < -EPS || (val_f.round() - val_f).abs() > EPS {
                // Row requires negative or non-integer solution
                return Err(assignment);
            }
            let val = val_f.round() as u16;
            assignment[i_missing[0]] = Some(val);
            if row == 0 {
                return Ok(assignment);
            } else {
                return match self.rec_search(mat, assignment, row - 1, a.len() - 1) {
                    Ok(assig) => Ok(assig),
                    Err(mut assig) => {
                        assig[i_missing[0]] = None;
                        Err(assig)
                    }
                };
            }
        }

        // More than one unspecified variable, try all options and find the smallest overall
        // solution.
        //
        let mut max_n = u32::MAX;
        let mut max_assignment = None;
        let search_col = *i_missing.last().unwrap();
        let search_button = self.buttons[search_col];
        // This button may not be pressed more than the least joltage it is connected to
        let search_bound = *self
            .joltages
            .iter()
            .enumerate()
            .filter(|(i, _)| (search_button & (1 << i)) != 0)
            .min()
            .unwrap()
            .1;
        let old_assignment = assignment.clone();
        for times in 0..=search_bound {
            assignment[search_col] = Some(times);
            assignment = match self.rec_search(mat, assignment, row, search_col - 1) {
                Ok(assig) => {
                    let value = assig.iter().map(|e| e.unwrap() as u32).sum();
                    if value < max_n {
                        max_n = value;
                        max_assignment = Some(assig);
                    }
                    old_assignment.clone()
                }
                Err(assig) => assig,
            };
        }
        if let Some(assig) = max_assignment {
            return Ok(assig);
        }
        assignment[search_col] = None;
        Err(assignment)
    }

    fn min_joltages(&self) -> u32 {
        let ab = self.matrix();

        //println!("{ab:?}");
        let ab = gauss_elim(ab);
        //println!("After Gauss elimination:\n{ab:?}");

        let assignment = vec![None; self.buttons.len()];
        let solution = self
            .rec_search(
                &ab,
                assignment,
                self.joltages.len() - 1,
                self.buttons.len() - 1,
            )
            .expect("No solution was found");
        let sol_vec: Vec<u16> = solution.into_iter().collect::<Option<Vec<u16>>>().unwrap();

        // Verify solution
        let x: Array1<f64> = sol_vec
            .iter()
            .map(|e| *e as f64)
            // Append -1 to subtract the target value. Then the result should be 0.
            .chain(std::iter::once(-1.0))
            .collect();
        let verify = ab.dot(&x);
        if !verify.iter().all(|e| e.abs() < EPS) {
            dbg!(verify);
            dbg!(sol_vec);
            panic!("INCORRECT SOLUTION");
        }

        // Return sum
        sol_vec.iter().map(|e| *e as u32).sum()
    }
}

fn part1(input: String) {
    let machines = parse_input(&input);
    let sum = machines.iter().map(|m| m.min_presses()).sum::<u32>();
    println!("{sum}");
}

fn part2(input: String) {
    let machines = parse_input(&input);
    let sum = machines.iter().map(|m| m.min_joltages()).sum::<u32>();
    println!("{sum}");
}

util::aoc_main!();
