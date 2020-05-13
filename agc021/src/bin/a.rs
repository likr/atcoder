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
        n: Chars,
    }
    let n = n
        .into_iter()
        .map(|c| c as usize - '0' as usize)
        .collect::<Vec<_>>();
    let m = n.len();

    let mut result = n.iter().sum::<usize>();
    for i in 0..m {
        let mut d = n.clone();
        if d[i] == 0 {
            continue;
        }
        d[i] -= 1;
        for j in i + 1..m {
            d[j] = 9;
        }
        let s = d.iter().sum::<usize>();
        result = max(result, s);
    }
    println!("{}", result);
}
