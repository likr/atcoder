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
        h: usize,
        w: usize,
        ab: [(Usize1, Usize1); h],
    }

    let mut start = BTreeMap::new();
    let mut moves = BTreeSet::new();
    for j in 0..w {
        start.insert(j, j);
        moves.insert((0, j));
    }
    for i in 0..h {
        let (ai, bi) = ab[i];
        let mut keys = vec![];
        for (&k, _) in start.range(ai..=bi) {
            keys.push(k);
        }
        if !start.contains_key(&(bi + 1)) && !keys.is_empty() && bi + 1 < w {
            start.insert(bi + 1, start[&keys[keys.len() - 1]]);
            moves.insert((bi - start[&(bi + 1)] + 1, bi + 1));
        }
        for &k in &keys {
            let v = start[&k];
            start.remove(&k);
            moves.remove(&(k - v, k));
        }
        // eprintln!("{:?}", start);
        // eprintln!("{:?}", moves);
        if let Some((d, _)) = moves.range(..).nth(0) {
            println!("{}", i + d + 1);
        } else {
            println!("-1");
        }
    }
}
