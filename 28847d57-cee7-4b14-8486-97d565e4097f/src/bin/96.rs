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
        n: usize,
        m: usize,
        k: usize,
        a: [usize; n],
    }
    let mut set1 = BTreeSet::new();
    let mut set2 = BTreeSet::new();
    let mut s = 0;
    for i in 0..m {
        set2.insert((a[i], i));
    }
    for _ in 0..k {
        let x = set2.pop_first().unwrap();
        s += x.0;
        set1.insert(x);
    }
    let mut ans = vec![format!("{}", s)];
    for i in m..n {
        if set1.contains(&(a[i - m], i - m)) {
            s -= a[i - m];
            set1.remove(&(a[i - m], i - m));
            set2.insert((a[i], i));
            let x = set2.pop_first().unwrap();
            s += x.0;
            set1.insert(x);
        } else {
            set2.remove(&(a[i - m], i - m));
            s += a[i];
            set1.insert((a[i], i));
            let x = set1.pop_last().unwrap();
            s -= x.0;
            set2.insert(x);
        }
        ans.push(format!("{}", s));
    }
    println!("{}", ans.join(" "));
}
