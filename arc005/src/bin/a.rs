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
        mut w: [String; n],
    }
    let t = w[n - 1].chars().collect::<Vec<_>>();
    w[n - 1] = (0..t.len() - 1).map(|i| t[i]).collect::<String>();
    let mut count = 0;
    for i in 0..n {
        match &*w[i] {
            "TAKAHASHIKUN" | "Takahashikun" | "takahashikun" => {
                count += 1;
            }
            _ => {}
        }
    }
    println!("{}", count);
}
