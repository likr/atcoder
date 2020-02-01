use proconio::input;
use std::cmp::{max, min};

fn cost(l: &Vec<usize>, mask: usize, x: usize) -> usize {
    let mut n = 0;
    let mut s = 0;
    for i in 0..l.len() {
        if mask & 1 << i > 0 {
            s += l[i];
            n += 1;
        }
    }
    10 * (n - 1) + max(s, x) - min(s, x)
}

fn main() {
    input! {
      n: usize,
      a: usize,
      b: usize,
      c: usize,
      mut l: [usize; n],
    }

    let m = 2usize.pow(n as u32);
    let mut cost_a = vec![0; m];
    let mut cost_b = vec![0; m];
    let mut cost_c = vec![0; m];
    for mask in 1..m {
        cost_a[mask] = cost(&l, mask, a);
        cost_b[mask] = cost(&l, mask, b);
        cost_c[mask] = cost(&l, mask, c);
    }
    let mut result = 1000000000;
    for i in 1..m {
        for j in 1..m {
            if i & j > 0 {
                continue;
            }
            for k in 1..m {
                if k & i > 0 || k & j > 0 {
                    continue;
                }
                let d = cost_a[i] + cost_b[j] + cost_c[k];
                if d < result {
                    result = d;
                }
            }
        }
    }
    println!("{}", result);
}
