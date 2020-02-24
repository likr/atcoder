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
        s: Chars,
    }
    let n = s.len() + 1;
    let mut a = vec![0; n];
    if s[0] == '<' {
        let mut k = 0;
        while s[k] == '<' {
            a[k + 1] = a[k] + 1;
            if k == n - 2 {
                break;
            }
            k += 1;
        }
    }
    if s[n - 2] == '>' {
        let mut k = n - 2;
        while s[k] == '>' {
            a[k] = max(a[k], a[k + 1] + 1);
            if k == 0 {
                break;
            }
            k -= 1;
        }
    }
    for i in 1..n - 1 {
        if s[i - 1] == '>' && s[i] == '<' {
            let mut k = i - 1;
            while s[k] == '>' {
                a[k] = max(a[k], a[k + 1] + 1);
                if k == 0 {
                    break;
                }
                k -= 1;
            }
            let mut k = i;
            while s[k] == '<' {
                a[k + 1] = max(a[k + 1], a[k] + 1);
                if k == n - 2 {
                    break;
                }
                k += 1;
            }
        }
    }
    eprintln!("{:?}", a);
    println!("{}", a.iter().sum::<usize>());
}
