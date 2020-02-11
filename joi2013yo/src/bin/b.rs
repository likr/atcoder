use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::{max, min};
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
        a: [[usize; 3]; n],
    }
    let mut score = vec![0; n];
    for j in 0..3 {
        let mut count = HashMap::new();
        for i in 0..n {
            if !count.contains_key(&a[i][j]) {
                count.insert(a[i][j], 0);
            }
            *count.get_mut(&a[i][j]).unwrap() += 1;
        }
        for i in 0..n {
            if count[&a[i][j]] == 1 {
                score[i] += a[i][j];
            }
        }
    }
    for &s in &score {
        println!("{}", s);
    }
}
