use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use rand::prelude::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;
use std::time::*;

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

fn initial_guess(s: &Vec<Vec<char>>) -> Vec<(usize, usize, char)> {
    vec![]
}

fn update(i: usize, j: usize, c: char, s: &mut Vec<Vec<char>>) {
    let n = s.len();

    let c0 = s[i][j];
    let mut queue = VecDeque::new();
    s[i][j] = c;
    queue.push_back((i, j));
    while let Some((i, j)) = queue.pop_front() {
        let mut ij2 = vec![];
        if 0 < j {
            ij2.push((i, j - 1));
        }
        if j + 1 < n {
            ij2.push((i, j + 1));
        }
        if 0 < i {
            ij2.push((i - 1, j));
        }
        if i + 1 < n {
            ij2.push((i + 1, j));
        }
        for &(i2, j2) in &ij2 {
            if s[i2][j2] == c0 {
                s[i2][j2] = c;
                queue.push_back((i2, j2));
            }
        }
    }
}

fn choice_max(s: &Vec<Vec<char>>) -> Option<(usize, usize, char)> {
    let n = s.len();
    let index = |i, j| i * n + j;
    let mut components = Dsu::new(n * n);
    for i in 0..n {
        for j in 1..n {
            if s[i][j - 1] == s[i][j] {
                components.merge(index(i, j - 1), index(i, j));
            }
        }
    }
    for i in 1..n {
        for j in 0..n {
            if s[i - 1][j] == s[i][j] {
                components.merge(index(i - 1, j), index(i, j));
            }
        }
    }
    let count = color_count(s);
    let c = (0..9)
        .map(|d| (d + '1' as usize) as u8 as char)
        .max_by_key(|&c| count[c as usize - '1' as usize])
        .unwrap();
    let mut groups = components.groups();
    groups.sort_by_key(|g| g.len());
    for group in groups.iter().rev() {
        if !group.is_empty() {
            let k = group[0];
            let i = k / n;
            let j = k % n;
            if s[i][j] != c {
                return Some((i, j, c));
            }
        }
    }
    None
}

fn color_count(s: &Vec<Vec<char>>) -> Vec<usize> {
    let n = s.len();
    let mut count = vec![0; 9];
    for i in 0..n {
        for j in 0..n {
            count[s[i][j] as usize - '1' as usize] += 1;
        }
    }
    count
}

fn current_score(s: &Vec<Vec<char>>, sol: &Vec<(usize, usize, char)>) -> i64 {
    let count = color_count(&s);
    let max_size = *count.iter().max().unwrap() as i64;
    max_size * 100 - sol.len() as i64
}

fn calc_score(s: &Vec<Vec<char>>, sol: &Vec<(usize, usize, char)>) -> i64 {
    if sol.len() > 10000 {
        return 0;
    }
    let mut t = s.clone();
    for &(i, j, c) in sol {
        update(i, j, c, &mut t);
    }
    current_score(&t, sol)
}

fn main() {
    let start = SystemTime::now();
    let limit = Duration::from_millis(2500);
    // let mut rng = thread_rng();
    input! {
        _id: usize,
        n: usize,
        _k: usize,
        mut s: [Chars; n],
    }
    let mut sol = initial_guess(&s);
    while let Some((i, j, c)) = choice_max(&mut s) {
        if start.elapsed().unwrap() >= limit {
            break;
        }
        update(i, j, c, &mut s);
        sol.push((i, j, c));
    }
    let best_sol = sol.clone();
    println!("{}", best_sol.len());
    for &(yi, xi, ci) in &best_sol {
        println!("{} {} {}", yi + 1, xi + 1, ci);
    }
}
//https://github.com/rust-lang-ja/ac-library-rs

pub mod dsu {
    /// Implement (union by size) + (path compression)
    /// Reference:
    /// Zvi Galil and Giuseppe F. Italiano,
    /// Data structures and algorithms for disjoint set union problems
    pub struct Dsu {
        n: usize,
        // root node: -1 * component size
        // otherwise: parent
        parent_or_size: Vec<i32>,
    }

    impl Dsu {
        // 0 <= size <= 10^8 is constrained.
        pub fn new(size: usize) -> Self {
            Self {
                n: size,
                parent_or_size: vec![-1; size],
            }
        }
        pub fn merge(&mut self, a: usize, b: usize) -> usize {
            assert!(a < self.n);
            assert!(b < self.n);
            let (mut x, mut y) = (self.leader(a), self.leader(b));
            if x == y {
                return x;
            }
            if -self.parent_or_size[x] < -self.parent_or_size[y] {
                std::mem::swap(&mut x, &mut y);
            }
            self.parent_or_size[x] += self.parent_or_size[y];
            self.parent_or_size[y] = x as i32;
            x
        }

        pub fn same(&mut self, a: usize, b: usize) -> bool {
            assert!(a < self.n);
            assert!(b < self.n);
            self.leader(a) == self.leader(b)
        }
        pub fn leader(&mut self, a: usize) -> usize {
            assert!(a < self.n);
            if self.parent_or_size[a] < 0 {
                return a;
            }
            self.parent_or_size[a] = self.leader(self.parent_or_size[a] as usize) as i32;
            self.parent_or_size[a] as usize
        }
        pub fn size(&mut self, a: usize) -> usize {
            assert!(a < self.n);
            let x = self.leader(a);
            -self.parent_or_size[x] as usize
        }
        pub fn groups(&mut self) -> Vec<Vec<usize>> {
            let mut leader_buf = vec![0; self.n];
            let mut group_size = vec![0; self.n];
            for i in 0..self.n {
                leader_buf[i] = self.leader(i);
                group_size[leader_buf[i]] += 1;
            }
            let mut result = vec![Vec::new(); self.n];
            for i in 0..self.n {
                result[i].reserve(group_size[i]);
            }
            for i in 0..self.n {
                result[leader_buf[i]].push(i);
            }
            result
                .into_iter()
                .filter(|x| !x.is_empty())
                .collect::<Vec<Vec<usize>>>()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn dsu_works() {
            let mut d = Dsu::new(4);
            d.merge(0, 1);
            assert_eq!(d.same(0, 1), true);
            d.merge(1, 2);
            assert_eq!(d.same(0, 2), true);
            assert_eq!(d.size(0), 3);
            assert_eq!(d.same(0, 3), false);
            assert_eq!(d.groups(), vec![vec![0, 1, 2], vec![3]]);
        }
    }
}
use dsu::*;
