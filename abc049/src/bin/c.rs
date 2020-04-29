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
        s: Chars,
    }
    let n = s.len();
    let dream = "dream".chars().collect::<Vec<_>>();
    let erase = "erase".chars().collect::<Vec<_>>();
    let mut k = 0;
    while k < n {
        eprintln!("{}", k);
        if s[k] == 'd' {
            if k + 4 >= n {
                println!("NO");
                return;
            } else {
                if (0..5).any(|i| s[k + i] != dream[i]) {
                    println!("NO");
                    return;
                }
                if k + 6 < n && s[k + 5] == 'e' && s[k + 6] == 'r' {
                    if k + 7 < n && s[k + 7] == 'a' {
                        k += 5;
                    } else {
                        k += 7;
                    }
                } else {
                    k += 5;
                }
            }
        } else if s[k] == 'e' {
            if k + 4 >= n {
                println!("NO");
                return;
            } else {
                if (0..5).any(|i| s[k + i] != erase[i]) {
                    println!("NO");
                    return;
                }
                if k + 5 < n && s[k + 5] == 'r' {
                    k += 6;
                } else {
                    k += 5;
                }
            }
        } else {
            println!("NO");
            return;
        }
    }
    println!("YES");
}
