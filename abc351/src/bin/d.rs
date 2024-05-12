use ac_library::*;
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

fn main() {
    input! {
        h: usize,
        w: usize,
        s0: [Chars; h],
    }
    let mut s = vec![vec!['@'; w + 2]; h + 2];
    for i in 0..h {
        for j in 0..w {
            s[i + 1][j + 1] = s0[i][j];
        }
    }
    for i in 1..=h {
        for j in 1..=w {
            if s[i][j] == '.'
                && (s[i - 1][j] == '#'
                    || s[i + 1][j] == '#'
                    || s[i][j - 1] == '#'
                    || s[i][j + 1] == '#')
            {
                s[i][j] = 'x';
            }
        }
    }
    let mut dsu = Dsu::new((h + 2) * (w + 2));
    for i in 1..h {
        for j in 1..=w {
            if s[i][j] == '.' && s[i + 1][j] == '.' {
                dsu.merge(i * (w + 2) + j, (i + 1) * (w + 2) + j);
            }
        }
    }
    for i in 1..=h {
        for j in 1..w {
            if s[i][j] == '.' && s[i][j + 1] == '.' {
                dsu.merge(i * (w + 2) + j, i * (w + 2) + j + 1);
            }
        }
    }
    let mut ans = HashMap::new();
    for i in 1..=h {
        for j in 1..=w {
            if s[i][j] == '.' {
                ans.entry(dsu.leader(i * (w + 2) + j))
                    .or_insert(HashSet::new())
                    .insert((i, j));
            }
        }
    }

    for i in 1..=h {
        for j in 1..=w {
            if s[i][j] == 'x' {
                if let Some(s) = ans.get_mut(&dsu.leader((i - 1) * (w + 2) + j)) {
                    s.insert((i, j));
                }
                if let Some(s) = ans.get_mut(&dsu.leader((i + 1) * (w + 2) + j)) {
                    s.insert((i, j));
                }
                if let Some(s) = ans.get_mut(&dsu.leader(i * (w + 2) + j - 1)) {
                    s.insert((i, j));
                }
                if let Some(s) = ans.get_mut(&dsu.leader(i * (w + 2) + j + 1)) {
                    s.insert((i, j));
                }
            }
        }
    }
    println!("{}", ans.values().map(|s| s.len()).max().unwrap_or(1));
}
