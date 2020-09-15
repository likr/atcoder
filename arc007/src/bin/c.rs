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

macro_rules! debug {
    ($($a:expr),* $(,)*) => {
    #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn dfs(x: &Vec<char>, c: &Vec<char>, cache: &mut HashMap<String, usize>) -> usize {
    debug!(x);
    let sx = x.iter().collect::<String>();
    if let Some(&c) = cache.get(&sx) {
        return c;
    }
    let n = c.len();
    let mut result = INF;
    for offset in 0..n {
        let mut y = x.clone();
        let mut changed = 0;
        for i in offset..2 * n {
            if c[(i - offset) % n] == 'o' && y[i] == 'x' {
                y[i] = 'o';
                changed += 1;
            }
        }
        if changed > 0 {
            for i in 0..n {
                if (0..n).all(|j| y[i + j] == 'o') {
                    return 1;
                }
            }
            result = min(result, 1 + dfs(&y, c, cache));
        }
    }
    cache.insert(sx, result);
    return result;
}

fn main() {
    input! {
        c: Chars,
    }
    let mut cache = HashMap::new();
    let x = vec!['x'; 2 * c.len()];
    println!("{}", dfs(&x, &c, &mut cache));
}
