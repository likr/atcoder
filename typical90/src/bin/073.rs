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

fn rec(
    graph: &Vec<Vec<usize>>,
    u: usize,
    c: &Vec<char>,
    count: &mut HashMap<usize, (usize, usize)>,
) {
    if c[u] == 'a' {
        count.insert(u, (1, 0));
    } else {
        count.insert(u, (0, 1));
    }
    if graph[u].len() == 1 && count.contains_key(&graph[u][0]) {
        return;
    }
    for &v in graph[u].iter() {
        if !count.contains_key(&v) {
            rec(graph, v, c, count);
            count.get_mut(&u).unwrap().0 += count[&v].0;
            count.get_mut(&u).unwrap().1 += count[&v].1;
        }
    }
}

fn main() {
    input! {
        n: usize,
        c: [char; n],
        ab: [(Usize1, Usize1); n - 1],
    }
    let mut a_count = 0;
    for i in 0..n {
        if c[i] == 'a' {
            a_count += 1;
        }
    }
    if a_count == 0 || a_count == n {
        println!("0");
        return;
    }
    let mut graph = vec![vec![]; n];
    for &(ai, bi) in ab.iter() {
        graph[ai].push(bi);
        graph[bi].push(ai);
    }
    let mut count = HashMap::new();
    rec(&graph, 0, &c, &mut count);
    let mut result = 1;
    for &(u, v) in ab.iter() {
        let (ua, ub) = count[&u];
        let (va, vb) = count[&v];
        let (wa, wb) = count[&0];
        if wa - min(ua, va) > 0 && min(ua, va) > 0 && wb - min(ub, vb) > 0 && min(ub, vb) > 0 {
            result = (result * 2) % M;
        }
    }
    println!("{}", result);
}
