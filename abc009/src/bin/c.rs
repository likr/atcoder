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

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }
    let mut t = s.clone();
    t.sort();

    let mut result = vec![' '; n];
    let mut x = 0;
    for i in 0..n {
        for j in 0..t.len() {
            result[i] = t[j];
            if s[i] != result[i] {
                x += 1;
            }
            let mut chars = vec![];
            for k in 0..t.len() {
                if j != k {
                    chars.push(t[k]);
                }
            }
            for k in i + 1..n {
                if let Some(l) = chars.iter().position(|&d| s[k] == d) {
                    chars.remove(l);
                }
            }
            let y = chars.len();

            if x + y <= k {
                t.remove(t.iter().position(|&d| result[i] == d).unwrap());
                break;
            } else {
                if s[i] != result[i] {
                    x -= 1;
                }
            }
        }
    }
    println!("{}", result.iter().collect::<String>());
}
