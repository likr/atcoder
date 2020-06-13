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
    let mut horizontal = vec![vec![0; w]; h];
    for i in 0..h {
        let mut l = 0;
        while l < w {
            if s[i][l] == '.' {
                let mut r = l + 1;
                while r < w && s[i][r] == '.' {
                    r += 1;
                }
                for j in l..r {
                    horizontal[i][j] = r - l;
                }
                l = r;
            } else {
                l += 1;
            }
        }
    }
    // for i in 0..h {
    //     eprintln!("{:?}", horizontal[i]);
    // }

    let mut vertical = vec![vec![0; w]; h];
    for j in 0..w {
        let mut l = 0;
        while l < h {
            if s[l][j] == '.' {
                let mut r = l + 1;
                while r < h && s[r][j] == '.' {
                    r += 1;
                }
                for i in l..r {
                    vertical[i][j] = r - l;
                }
                l = r;
            } else {
                l += 1;
            }
        }
    }
    // for i in 0..h {
    //     eprintln!("{:?}", vertical[i]);
    // }
    let mut result = 0usize;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                result = max(result, horizontal[i][j] + vertical[i][j] - 1);
            }
        }
    }
    println!("{}", result);
}
