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
        s: Chars,
    }
    let mut r_count = s
        .iter()
        .map(|&c| if c == 'R' { 1 } else { 0 })
        .collect::<Vec<_>>();
    let mut g_count = s
        .iter()
        .map(|&c| if c == 'G' { 1 } else { 0 })
        .collect::<Vec<_>>();
    let mut b_count = s
        .iter()
        .map(|&c| if c == 'B' { 1 } else { 0 })
        .collect::<Vec<_>>();
    for i in 1..n {
        r_count[i] += r_count[i - 1];
        g_count[i] += g_count[i - 1];
        b_count[i] += b_count[i - 1];
    }
    eprintln!("{:?}", r_count);
    eprintln!("{:?}", g_count);
    eprintln!("{:?}", b_count);
    let mut result = 0usize;
    for i in 0..n {
        for j in i + 1..n {
            if s[i] == 'R' && s[j] == 'G' || s[i] == 'G' && s[j] == 'R' {
                if 2 * j - i < n && s[2 * j - i] == 'B' {
                    result += b_count[n - 1] - b_count[j] - 1;
                } else {
                    result += b_count[n - 1] - b_count[j];
                }
            } else if s[i] == 'R' && s[j] == 'B' || s[i] == 'B' && s[j] == 'R' {
                if 2 * j - i < n && s[2 * j - i] == 'G' {
                    result += g_count[n - 1] - g_count[j] - 1;
                } else {
                    result += g_count[n - 1] - g_count[j];
                }
            } else if s[i] == 'B' && s[j] == 'G' || s[i] == 'G' && s[j] == 'B' {
                if 2 * j - i < n && s[2 * j - i] == 'R' {
                    result += r_count[n - 1] - r_count[j] - 1;
                } else {
                    result += r_count[n - 1] - r_count[j];
                }
            }
        }
    }
    println!("{}", result);
}
