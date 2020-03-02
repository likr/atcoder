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
        s: [Chars; h],
    }
    let mut t = vec![vec![INF; w]; h];
    let p = vec![
        (0, 0),
        (0, 1),
        (0, 2),
        (1, 0),
        (1, 2),
        (2, 0),
        (2, 1),
        (2, 2),
    ];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                continue;
            }
            let mut c = 0;
            for &(dyi, dxi) in &p {
                if 1 <= j + dxi && j + dxi < w + 1 && 1 <= i + dyi && i + dyi < h + 1 {
                    if s[i + dyi - 1][j + dxi - 1] == '#' {
                        c += 1;
                    }
                }
            }
            t[i][j] = c;
        }
    }
    for i in 0..h {
        for j in 0..w {
            if t[i][j] == INF {
                print!("#");
            } else {
                print!("{}", t[i][j]);
            }
        }
        println!("");
    }
}
