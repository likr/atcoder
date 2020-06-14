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
        m: usize,
        q: usize,
    }
    let mut score = vec![n; m];
    let mut solved = vec![vec![]; n];
    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            input! {
                a: Usize1,
            }
            let s = solved[a].iter().map(|&k| score[k]).sum::<usize>();
            println!("{}", s);
        } else {
            input! {
                a: Usize1,
                b: Usize1,
            }
            solved[a].push(b);
            score[b] -= 1;
        }
    }
}
