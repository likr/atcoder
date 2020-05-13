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
        a: [usize; n],
        b: [usize; n],
    }
    let mut result = 0;
    let mut r = vec![];
    let mut s = 0;
    for i in 0..n {
        if a[i] > b[i] {
            r.push(a[i] - b[i]);
        } else if b[i] > a[i] {
            result += 1;
            s += b[i] - a[i];
        }
    }
    eprintln!("{} {:?}", s, r);
    if r.iter().sum::<usize>() < s {
        println!("-1");
        return;
    }
    if s == 0 {
        println!("0");
        return;
    }

    r.sort();
    r.reverse();
    for i in 0..r.len() {
        result += 1;
        if r[i] >= s {
            break;
        }
        s -= r[i];
    }
    println!("{}", result);
}
