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

fn find_path(
    s: usize,
    h: usize,
    w: usize,
    events: &Vec<(usize, usize, usize)>,
) -> (usize, Vec<(usize, usize)>) {
    let mut index = 0;
    let mut d = 0;
    let mut walls = BTreeSet::new();
    let mut path = vec![(0, 0)];
    walls.insert(h);
    for k in 0..s {
        while index < events.len() && events[index].0 == k {
            let (_, close, j) = events[index];
            if close == 1 {
                walls.remove(&j);
            } else {
                walls.insert(j);
            }
            index += 1;
        }
    }
    for k in s..w {
        while index < events.len() && events[index].0 == k && events[index].1 == 0 {
            let (_, _, j) = events[index];
            walls.insert(j);
            index += 1;
        }
        let new_d = *walls.range(d..).nth(0).unwrap();
        if new_d > d {
            path.push((k, new_d));
            d = new_d;
        }
        while index < events.len() && events[index].0 == k && events[index].1 == 1 {
            let (_, _, j) = events[index];
            walls.remove(&j);
            index += 1;
        }
    }
    (d, path)
}

fn main() {
    input! {
        h: usize,
        w: usize,
        ab: [(Usize1, Usize1); h],
    }
    let mut events = vec![];
    for j in 0..h {
        let (aj, bj) = ab[j];
        events.push((aj, 0, j));
        events.push((bj, 1, j));
    }
    events.sort();
    let (y0, mut ans_path) = find_path(0, h, w, &events);
    debug!(y0);
    debug!(ans_path);
    let mut ok = 0;
    let mut ng = w;
    while ng - ok > 1 {
        let s = (ok + ng) / 2;
        let (y, path) = find_path(s, h, w, &events);
        if y == y0 {
            ok = s;
            ans_path = path;
        } else {
            ng = s;
        }
    }
    for i in 1..ans_path.len() {
        let (_, y1) = ans_path[i - 1];
        let (x2, y2) = ans_path[i];
        for j in (y1 + 1)..=y2 {
            println!("{}", x2 - ok + j);
        }
    }
    for _ in y0 + 1..=h {
        println!("-1");
    }
}
