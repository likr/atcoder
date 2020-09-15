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
    }
    let n = a.len();
    let mut diff = vec![];
    let mut count_a = vec![0; 26];
    let mut count_b = vec![0; 26];
    for i in 0..n {
        if a[i] != b[i] {
            diff.push((a[i], b[i]));
        }
        count_a[a[i] as usize - 'a' as usize] += 1;
        count_b[b[i] as usize - 'a' as usize] += 1;
    }

    let m = diff.len();
    if m > 6 || count_a != count_b {
        println!("NO");
        return;
    }

    let mut ij = vec![];
    for j in 0..m {
        for i in 0..j {
            ij.push((i, j));
        }
    }

    let t = diff.iter().map(|&(_, c)| c).collect::<Vec<char>>();
    if count_a.iter().any(|&c| c >= 2) {
        if m == 0 {
            println!("YES");
            return;
        }
        for iter in 1..=3 {
            for mut x in 0..ij.len().pow(iter) {
                let mut s = diff.iter().map(|&(c, _)| c).collect::<Vec<char>>();
                for _ in 0..iter {
                    let (i, j) = ij[x % ij.len()];
                    s.swap(i, j);
                    x /= ij.len();
                }
                if t == s {
                    println!("YES");
                    return;
                }
            }
        }
    } else {
        for mut x in 0..ij.len().pow(3) {
            let mut s = diff.iter().map(|&(c, _)| c).collect::<Vec<char>>();
            for _ in 0..3 {
                let (i, j) = ij[x % ij.len()];
                s.swap(i, j);
                x /= ij.len();
            }
            if t == s {
                println!("YES");
                return;
            }
        }
    }

    println!("NO");
}
