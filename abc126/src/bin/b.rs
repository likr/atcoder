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
        s: Chars,
    }
    let a = format!("{}{}", s[0], s[1]).parse::<usize>().unwrap();
    let b = format!("{}{}", s[2], s[3]).parse::<usize>().unwrap();
    if a < 1 || 12 < a {
        if b < 1 || 12 < b {
            println!("NA");
        } else {
            println!("YYMM");
        }
    } else {
        if b < 1 || 12 < b {
            println!("MMYY");
        } else {
            println!("AMBIGUOUS");
        }
    }
}
