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

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn dist2(x1: usize, y1: usize, x2: usize, y2: usize) -> usize {
    let dx = max(x1, x2) - min(x1, x2);
    let dy = max(y1, y2) - min(y1, y2);
    dx * dx + dy * dy
}

fn main() {
    let n = 9usize;
    input! {
        s: [Chars; n],
    }
    let mut pos = vec![];
    for x in 0..n {
        for y in 0..n {
            pos.push((x, y));
        }
    }

    let mut count = 0usize;
    for i in 0..pos.len() {
        for j in 0..i {
            for k in 0..j {
                for l in 0..k {
                    let (x1, y1) = pos[i];
                    let (x2, y2) = pos[j];
                    let (x3, y3) = pos[k];
                    let (x4, y4) = pos[l];
                    let mut d = vec![
                        dist2(x1, y1, x2, y2),
                        dist2(x1, y1, x3, y3),
                        dist2(x1, y1, x4, y4),
                        dist2(x2, y2, x3, y3),
                        dist2(x2, y2, x4, y4),
                        dist2(x3, y3, x4, y4),
                    ];
                    d.sort();
                    if d[0] == d[1]
                        && d[1] == d[2]
                        && d[2] == d[3]
                        && d[3] != d[4]
                        && d[4] == d[5]
                        && s[x1][y1] == '#'
                        && s[x2][y2] == '#'
                        && s[x3][y3] == '#'
                        && s[x4][y4] == '#'
                    {
                        count += 1;
                    }
                }
            }
        }
    }

    println!("{}", count);
}
