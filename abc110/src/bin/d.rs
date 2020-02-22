use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;

#[allow(unused)]
const INF: usize = std::usize::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

fn inv(a: usize) -> usize {
    let m = M as i64;
    let mut a = a as i64;
    let mut b = m as i64;
    let mut u = 1;
    let mut v = 0;
    let mut tmp;
    while b != 0 {
        let t = a / b;
        a -= t * b;
        tmp = a;
        a = b;
        b = tmp;
        u -= t * v;
        tmp = u;
        u = v;
        v = tmp;
    }
    u %= m;
    if u < 0 {
        u += m;
    }
    return u as usize;
}

fn factors(n: usize) -> Vec<usize> {
    let sqrt_n = (n as f64).sqrt() as usize;
    let mut ret = HashSet::new();
    for d in 1..=sqrt_n {
        if n % d == 0 {
            ret.insert(d);
            ret.insert(n / d);
        }
    }
    let mut ret = ret.into_iter().collect::<Vec<_>>();
    ret.sort();
    ret
}

fn dfs(
    n: usize,
    m: usize,
    factors: &HashMap<usize, Vec<usize>>,
    ret: &mut Vec<usize>,
    f: &Vec<usize>,
    g: &Vec<usize>,
) -> usize {
    let depth = ret.len();
    if depth > n {
        return 0;
    }
    if m == 1 {
        let mut count = HashMap::new();
        for &v in ret.iter() {
            *count.entry(v).or_insert(0) += 1;
        }
        count.insert(1, n - depth);
        let mut c = f[n];
        for &ci in count.values() {
            c = c * g[ci] % M;
        }
        // eprintln!("{:?} {}", ret, c);
        return c;
    }
    let mut c = 0;
    for &x in factors.get(&m).unwrap() {
        if x == 1 || (depth > 0 && x > ret[depth - 1]) {
            continue;
        }
        ret.push(x);
        c = (c + dfs(n, m / x, factors, ret, f, g)) % M;
        ret.resize(depth, 0);
    }
    c
}

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let m_factors = factors(m);
    // eprintln!("{:?}", m_factors);
    let all_factors = m_factors
        .iter()
        .map(|&f| (f, factors(f)))
        .collect::<HashMap<_, _>>();
    // eprintln!("{:?}", all_factors);

    let mut f = vec![1; n + 1];
    let mut g = vec![1; n + 1];
    for i in 1..f.len() {
        f[i] = i * f[i - 1] % M;
        g[i] = inv(f[i]);
    }
    println!("{}", dfs(n, m, &all_factors, &mut vec![], &f, &g));
}
