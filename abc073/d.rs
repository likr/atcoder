use std::collections::HashMap;

macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn wf(graph: &Vec<Vec<(usize, usize)>>) -> Vec<Vec<usize>> {
    let n = graph.len();
    let mut result = vec![vec![200000000 as usize; n]; n];
    for i in 0..n {
        result[i][i] = 0;
        for &(j, c) in graph[i].iter() {
            result[i][j] = c;
            result[j][i] = c;
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                let d = result[i][k] + result[k][j];
                if d < result[i][j] {
                    result[i][j] = d;
                }
            }
        }
    }
    result
}

pub trait LexicalPermutation {
    /// Return `true` if the slice was permuted, `false` if it is already
    /// at the last ordered permutation.
    fn next_permutation(&mut self) -> bool;
    /// Return `true` if the slice was permuted, `false` if it is already
    /// at the first ordered permutation.
    fn prev_permutation(&mut self) -> bool;
}

impl<T> LexicalPermutation for [T]
where
    T: Ord,
{
    /// Original author in Rust: Thomas Backman <serenity@exscape.org>
    fn next_permutation(&mut self) -> bool {
        // These cases only have 1 permutation each, so we can't do anything.
        if self.len() < 2 {
            return false;
        }

        // Step 1: Identify the longest, rightmost weakly decreasing part of the vector
        let mut i = self.len() - 1;
        while i > 0 && self[i - 1] >= self[i] {
            i -= 1;
        }

        // If that is the entire vector, this is the last-ordered permutation.
        if i == 0 {
            return false;
        }

        // Step 2: Find the rightmost element larger than the pivot (i-1)
        let mut j = self.len() - 1;
        while j >= i && self[j] <= self[i - 1] {
            j -= 1;
        }

        // Step 3: Swap that element with the pivot
        self.swap(j, i - 1);

        // Step 4: Reverse the (previously) weakly decreasing part
        self[i..].reverse();

        true
    }

    fn prev_permutation(&mut self) -> bool {
        // These cases only have 1 permutation each, so we can't do anything.
        if self.len() < 2 {
            return false;
        }

        // Step 1: Identify the longest, rightmost weakly increasing part of the vector
        let mut i = self.len() - 1;
        while i > 0 && self[i - 1] <= self[i] {
            i -= 1;
        }

        // If that is the entire vector, this is the first-ordered permutation.
        if i == 0 {
            return false;
        }

        // Step 2: Reverse the weakly increasing part
        self[i..].reverse();

        // Step 3: Find the rightmost element equal to or bigger than the pivot (i-1)
        let mut j = self.len() - 1;
        while j >= i && self[j - 1] < self[i - 1] {
            j -= 1;
        }

        // Step 4: Swap that element with the pivot
        self.swap(i - 1, j);

        true
    }
}

fn main() {
    input! {
        N: usize,
        M: usize,
        R: usize,
        r: [usize; R],
        abc: [(usize, usize, usize); M],
    }
    let mut adjacencies: Vec<Vec<(usize, usize)>> = vec![vec![]; N + 1];
    for (a, b, c) in abc {
        adjacencies[a].push((b, c));
    }
    let d = wf(&adjacencies);
    let mut r = r;
    r.sort();

    let mut result = std::usize::MAX;
    loop {
        let mut dist = 0;
        let mut prev: usize = r[0];
        for i in 1..R {
            dist += d[prev][r[i]];
            prev = r[i];
        }
        // println!("{:?} {}", r, dist);
        if dist < result {
            result = dist;
        }
        if !r.next_permutation() {
            break;
        }
    }
    println!("{}", result);
}
