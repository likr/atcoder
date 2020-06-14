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
        a: [Chars; n],
    }
    let mut chars = vec![];
    'outer: for k in 0..n / 2 {
        for i in 0..n {
            for j in 0..n {
                if a[k][i] == a[n - 1 - k][j] {
                    chars.push(a[k][i]);
                    continue 'outer;
                }
            }
        }
        println!("-1");
        return;
    }
    for &c in &chars {
        print!("{}", c);
    }
    if n % 2 == 1 {
        print!("{}", a[n / 2][0]);
    }
    chars.reverse();
    for &c in &chars {
        print!("{}", c);
    }
}
