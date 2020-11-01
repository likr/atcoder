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

fn check(s: &Vec<usize>) -> bool {
    let n = s.len();
    let mut count = vec![0usize; 10];
    for &c in s {
        count[c] += 1;
    }

    if n == 1 {
        return s[0] == 8;
    } else if n == 2 {
        for a in 1..=9 {
            for b in 1..=9 {
                if (a + 10 * b) % 8 == 0 {
                    if (a == b && count[a] >= 2) || (a != b && count[a] >= 1 && count[b] >= 1) {
                        return true;
                    }
                }
            }
        }
    } else {
        for c in 1..=9 {
            for b in 1..=9 {
                for a in 1..=9 {
                    if (a + 10 * b + 100 * c) % 8 == 0 {
                        if a == b && b == c {
                            if count[a] >= 3 {
                                return true;
                            }
                        } else if a == b {
                            if count[a] >= 2 && count[c] >= 1 {
                                return true;
                            }
                        } else if a == c {
                            if count[a] >= 2 && count[b] >= 1 {
                                return true;
                            }
                        } else if b == c {
                            if count[b] >= 2 && count[a] >= 1 {
                                return true;
                            }
                        } else {
                            if count[a] >= 1 && count[b] >= 1 && count[c] >= 1 {
                                return true;
                            }
                        }
                    }
                }
            }
        }
    }
    return false;
}

fn main() {
    input! {
        s: Chars,
    }
    let s = s
        .into_iter()
        .map(|ci| ci as usize - '0' as usize)
        .collect::<Vec<_>>();
    // let mut b = vec![1; n];
    // for i in 1..n {
    //     b[i] = b[i - 1] * 10 % 8;
    // }
    // debug!(b);
    //
    if check(&s) {
        println!("Yes");
    } else {
        println!("No");
    }
}
