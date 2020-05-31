use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;

#[allow(unused)]
const INF: usize = std::usize::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

fn index(a: usize, b: usize, c: usize) -> usize {
    a * 9 + b * 3 + c
}

fn abc(i: usize) -> (usize, usize, usize) {
    (i / 9, i % 9 / 3, i % 3)
}

fn main() {
    input! {
        n: usize,
        mut a: usize,
        mut b: usize,
        mut c: usize,
        s: [String; n],
    }
    let mut states = vec![vec![None; 27]; n + 1];
    states[0][index(min(2, a), min(2, b), min(2, c))] = Some((' ', INF));
    for i in 1..=n {
        let mut available = false;
        for j in 0..27 {
            if let Some(_) = states[i - 1][j] {
                let (aj, bj, cj) = abc(j);
                if &*s[i - 1] == "AB" {
                    if bj > 0 {
                        states[i][index(min(2, aj + 1), bj - 1, cj)] = Some(('A', j));
                        available = true;
                    }
                    if aj > 0 {
                        states[i][index(aj - 1, min(2, bj + 1), cj)] = Some(('B', j));
                        available = true;
                    }
                }
                if &*s[i - 1] == "AC" {
                    if cj > 0 {
                        states[i][index(min(2, aj + 1), bj, cj - 1)] = Some(('A', j));
                        available = true;
                    }
                    if aj > 0 {
                        states[i][index(aj - 1, bj, min(2, cj + 1))] = Some(('C', j));
                        available = true;
                    }
                }
                if &*s[i - 1] == "BC" {
                    if cj > 0 {
                        states[i][index(aj, min(2, bj + 1), cj - 1)] = Some(('B', j));
                        available = true;
                    }
                    if bj > 0 {
                        states[i][index(aj, bj - 1, min(2, cj + 1))] = Some(('C', j));
                        available = true;
                    }
                }
            }
        }
        if !available {
            println!("No");
            return;
        }
    }

    println!("Yes");
    let (d, mut j) = states[n]
        .iter()
        .filter(|o| o.is_some())
        .nth(0)
        .unwrap()
        .unwrap();
    let mut sol = vec![d];
    for i in (1..n).rev() {
        let (e, k) = states[i][j].unwrap();
        sol.push(e);
        j = k;
    }
    sol.reverse();
    for i in 0..n {
        let d = sol[i];
        // if &*s[i] == "AB" {
        //     if d == 'A' {
        //         a += 1;
        //         b -= 1;
        //     } else {
        //         a -= 1;
        //         b += 1;
        //     }
        // }
        // if &*s[i] == "AC" {
        //     if d == 'A' {
        //         a += 1;
        //         c -= 1;
        //     } else {
        //         a -= 1;
        //         c += 1;
        //     }
        // }
        // if &*s[i] == "BC" {
        //     if d == 'B' {
        //         b += 1;
        //         c -= 1;
        //     } else {
        //         b -= 1;
        //         c += 1;
        //     }
        // }
        // eprintln!("{} {} {}", a, b, c);
        println!("{}", d);
    }
}
