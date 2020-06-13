use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
use segment_tree::ops::Add;
use segment_tree::PointSegment;
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
        n: usize,
        q: usize,
        mut regions: [(isize, isize, isize, isize); n],
        ab: [(isize, isize); q],
    }
    regions.sort();
    let mut q_index = (0..q).collect::<Vec<_>>();
    q_index.sort_by_key(|&i| ab[i]);

    let mut ys = vec![];
    for i in 0..n {
        let (_, yi, di, _) = regions[i];
        ys.push(yi);
        ys.push(yi + di);
    }
    for i in 0..q {
        let (_, bi) = ab[i];
        ys.push(bi);
    }
    ys.sort();
    ys.dedup();
    let mut index = HashMap::new();
    for i in 0..ys.len() {
        index.insert(ys[i], i);
    }

    let mut events = BinaryHeap::new();
    for i in 0..n {
        let (xi, _, di, _) = regions[i];
        events.push(Reverse((xi, i, false)));
        events.push(Reverse((xi + di + 1, i, true)));
    }

    let mut result = vec![0; q];
    let mut tree = PointSegment::build(vec![0isize; ys.len()], Add);
    for &i in &q_index {
        let (ai, bi) = ab[i];
        while let Some(&Reverse((t, j, remove))) = events.peek() {
            if t > ai {
                break;
            }
            let (_, yj, dj, cj) = regions[j];
            if remove {
                tree.modify(index[&yj], index[&(yj + dj)] + 1, -cj);
            } else {
                tree.modify(index[&yj], index[&(yj + dj)] + 1, cj);
            }
            events.pop();
        }
        result[i] = tree.query(index[&bi]);
    }
    for i in 0..q {
        println!("{}", result[i]);
    }
}
