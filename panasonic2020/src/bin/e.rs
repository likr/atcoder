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
        a: Chars,
        b: Chars,
        c: Chars,
    }
    let na = a.len();
    let nb = b.len();
    let nc = c.len();

    let offset = na + nb + nc;
    let size = 2 * offset;
    let mut ab = vec![true; size];
    let mut ac = vec![true; size];
    let mut bc = vec![true; size];

    for i in 0..na {
        for j in 0..nb {
            if a[i] != '?' && b[j] != '?' && a[i] != b[j] {
                ab[offset + i - j] = false;
            }
        }
    }

    for i in 0..na {
        for k in 0..nc {
            if a[i] != '?' && c[k] != '?' && a[i] != c[k] {
                ac[offset + i - k] = false;
            }
        }
    }

    for j in 0..nb {
        for k in 0..nc {
            if b[j] != '?' && c[k] != '?' && b[j] != c[k] {
                bc[offset + j - k] = false;
            }
        }
    }

    let mut result = INF;
    for j in 0..size {
        for k in 0..size {
            if offset + k < size + j && j <= offset + k && ab[j] && ac[k] && bc[offset + k - j] {
                let left = min(offset, min(j, k));
                let right = max(offset + na, max(j + nb, k + nc));
                result = min(result, right - left);
            }
        }
    }
    println!("{}", result);
}
