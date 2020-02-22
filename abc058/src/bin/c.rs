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
        s: [Chars; n],
    }
    let mut count = vec![vec![0; 26]; n];
    for i in 0..n {
        for &c in &s[i] {
            count[i][c as usize - 'a' as usize] += 1;
        }
    }
    for j in 0..26 {
        let m = count.iter().map(|row| row[j]).min().unwrap();
        for _ in 0..m {
            print!("{}", (j + 'a' as usize) as u8 as char);
        }
    }
    println!("");
}
