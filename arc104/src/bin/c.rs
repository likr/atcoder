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

fn solve(ab: &Vec<(Option<usize>, Option<usize>)>) -> bool {
    let n = ab.len();
    let mut any_count = 0;
    let mut visited = HashSet::new();
    for i in 0..n {
        let (ai, bi) = ab[i];
        if ai.is_none() && bi.is_none() {
            any_count += 1;
        }
        if (ai.is_some() && bi.is_some() && ai.unwrap() >= bi.unwrap())
            || visited.contains(&ai.unwrap())
            || visited.contains(&bi.unwrap())
        {
            eprintln!("invalid input {} {:?} {:?}", i, ai, bi);
            return false;
        }
        if let Some(ai) = ai {
            visited.insert(ai);
        }
        if let Some(bi) = bi {
            visited.insert(bi);
        }
    }

    let mut c = vec![0; n];
    for i in 0..n {
        let (ai, bi) = ab[i];
        for j in 0..n {
            let (aj, bj) = ab[j];
            if i != j {
                if ai < aj && aj < bi {
                    c[i] += 1;
                }
                if ai < bj && bj < bi {
                    c[i] += 1;
                }
            }
        }
    }
    debug!(c);

    let mut conditions = vec![];
    for i in 0..n {
        let (ai, bi) = ab[i];
        if let Some(ai) = ai {
            if let Some(bi) = bi {
                conditions.push((ai, bi));
            }
        }
    }

    let mut used = vec![false; 2 * n + 1];
    for i in 0..n {
        let (ai, bi) = ab[i];
        if let Some(ai) = ai {
            used[ai as usize] = true;
        }
        if let Some(bi) = bi {
            used[bi as usize] = true;
        }
    }

    let mut min_c = vec![INF as i64; n];
    for i in 0..n {
        let (ai, bi) = ab[i];
        if ai != -1 && bi != -1 {
            min_c[i] = bi - ai - 1;
        } else if ai != -1 {
            for x in 1..bi {
                if !used[x as usize] {
                    min_c[i] = min(min_c[i], bi - x - 1);
                }
            }
        } else if bi != -1 {
            for x in ai + 1..=2 * n as i64 {
                if !used[x as usize] {
                    min_c[i] = min(min_c[i], x - ai - 1);
                }
            }
        }
    }

    let mut x_count = 0;
    for &(ak, bk) in &conditions {
        let ck = bk - ak - 1;
        if ck == 0 {
            continue;
        }
        let mut ok = false;
        for l in 0..=ck {}
        if !ok {
            return false;
        }
        let mut up_count = 0;
        let mut down_count = 0;
        for i in 0..n {
            let (ai, bi) = ab[i];
            if bi < ak - ck || bk + ck < ai {
                continue;
            }
            if ai != -1 && ak < ai && ai < bk {
                if bi != -1 && bi - ai - 1 != ck {
                    return false;
                }
                up_count += 1;
            }
            if bi != -1 && ak < bi && bi < bk {
                if bi != -1 && bi - ai - 1 != ck {
                    return false;
                }
                down_count += 1;
            }
        }
        if up_count + down_count != ck {
            debug!(ak, bk, up_count, down_count);
            return false;
        }
    }

    let mut indices = vec![None; 2 * n];

    true
}

fn main() {
    input! {
        n: usize,
        ab: [(i64, i64); n],
    }
    let ab = ab
        .into_iter()
        .map(|(ai, bi)| {
            (
                if ai == -1 {
                    None
                } else {
                    Some(ai as usize - 1)
                },
                if bi == -1 {
                    None
                } else {
                    Some(bi as usize - 1)
                },
            )
        })
        .collect::<Vec<_>>();
    if solve(&ab) {
        println!("Yes");
    } else {
        println!("No");
    }
}
