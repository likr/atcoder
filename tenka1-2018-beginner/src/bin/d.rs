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
    }
    let mut d = 1;
    while d * d < 1 + 8 * n {
        d += 1;
    }
    if d * d != 1 + 8 * n {
        println!("No");
        return;
    }
    let k = (1 + d) / 2;
    let mut s = vec![vec![]; k];
    let mut x = 1;
    for i in 0..k {
        for j in 0..i {
            s[i].push(x);
            s[j].push(x);
            x += 1;
        }
    }

    println!("Yes");
    println!("{}", k);
    for i in 0..k {
        print!("{} ", s[i].len());
        for &sij in &s[i] {
            print!("{} ", sij);
        }
        println!("");
    }
}
