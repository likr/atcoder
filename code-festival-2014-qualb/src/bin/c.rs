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
        s: [Chars; 3],
    }
    let n = s[0].len() / 2;
    let s = s
        .into_iter()
        .map(|si| {
            si.into_iter()
                .map(|c| c as usize - 'A' as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut c = vec![vec![0isize; 26]; 3];
    for j in 0..3 {
        for i in 0..2 * n {
            c[j][s[j][i]] += 1;
        }
        eprintln!("{:?}", c[j]);
    }
    let mut plus = 0;
    let mut minus = 0;
    for i in 0..26 {
        plus += min(c[0][i], c[2][i]);
        minus += max(0, c[2][i] - c[1][i]);
    }
    eprintln!("{} {}", plus, minus);
    if minus <= n as isize && n as isize <= plus {
        println!("YES");
    } else {
        println!("NO");
    }
}
