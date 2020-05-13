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
        h: usize,
        w: usize,
        c: [[usize; w]; h],
    }
    let mut black = vec![vec![0; w + 1]; h + 1];
    let mut white = vec![vec![0; w + 1]; h + 1];

    for i in 1..=h {
        for j in 1..=w {
            black[i][j] = black[i - 1][j] + black[i][j - 1] - black[i - 1][j - 1];
            white[i][j] = white[i - 1][j] + white[i][j - 1] - white[i - 1][j - 1];
            if (i + j) % 2 == 0 {
                black[i][j] += c[i - 1][j - 1];
            } else {
                white[i][j] += c[i - 1][j - 1];
            }
        }
    }
    // eprintln!("{:?}", black);
    // eprintln!("{:?}", white);

    let mut result = 0;
    for i1 in 1..=h {
        for j1 in 1..=w {
            for i0 in 1..=i1 {
                for j0 in 1..=j1 {
                    let s_b = black[i1][j1] + black[i0 - 1][j0 - 1]
                        - black[i0 - 1][j1]
                        - black[i1][j0 - 1];
                    let s_w = white[i1][j1] + white[i0 - 1][j0 - 1]
                        - white[i0 - 1][j1]
                        - white[i1][j0 - 1];
                    if s_b == s_w {
                        result = max(result, (i1 - i0 + 1) * (j1 - j0 + 1));
                    }
                }
            }
        }
    }
    println!("{}", result);
}
