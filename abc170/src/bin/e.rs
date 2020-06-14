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
        n: usize,
        q: usize,
        ab: [(usize, Usize1); n],
        cd: [(Usize1, Usize1); q],
    }
    let m = 200000;
    let mut place = vec![0; n];
    let mut players = vec![BTreeSet::new(); m];
    for i in 0..n {
        let (ai, bi) = ab[i];
        place[i] = bi;
        players[bi].insert(Reverse((ai, i)));
    }
    let mut max_scores = BTreeSet::new();
    for j in 0..m {
        if !players[j].is_empty() {
            let Reverse((ai, i)) = *players[j].range(..).nth(0).unwrap();
            max_scores.insert((ai, i));
        }
    }
    for &(c, d) in &cd {
        let x = place[c];
        let y = d;
        place[c] = d;
        let Reverse((x_max, x_max_i)) = *players[x].range(..).nth(0).unwrap();
        max_scores.remove(&(x_max, x_max_i));
        if !players[y].is_empty() {
            let Reverse((y_max, y_max_i)) = *players[y].range(..).nth(0).unwrap();
            max_scores.remove(&(y_max, y_max_i));
        }
        players[x].remove(&Reverse((ab[c].0, c)));
        players[y].insert(Reverse((ab[c].0, c)));
        if !players[x].is_empty() {
            let Reverse((new_x_max, new_x_max_i)) = *players[x].range(..).nth(0).unwrap();
            max_scores.insert((new_x_max, new_x_max_i));
        }
        let Reverse((new_y_max, new_y_max_i)) = *players[y].range(..).nth(0).unwrap();
        max_scores.insert((new_y_max, new_y_max_i));
        // eprintln!("{:?}", max_scores);
        println!("{}", max_scores.range(..).nth(0).unwrap().0);
    }
}
