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
        _n: usize,
        r: usize,
        mut s: Chars,
    }
    if let Some(n) = s.iter().rposition(|&c| c == '.') {
        let n = n + 1;
        let mut result = 0;
        for i in 0..n {
            if i + r >= n {
                for j in i..n {
                    s[j] = 'o'
                }
                result += 1;
                break;
            } else if s[i] == '.' {
                for j in i..i + r {
                    s[j] = 'o'
                }
                result += 1;
            }
            result += 1;
        }
        // eprintln!("{:?}", s);
        println!("{}", result);
    } else {
        println!("0");
    }
}
