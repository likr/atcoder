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

pub trait LexicalPermutation {
    fn next_permutation(&mut self) -> bool;
    fn prev_permutation(&mut self) -> bool;
}

impl<T> LexicalPermutation for [T]
where
    T: Ord,
{
    fn next_permutation(&mut self) -> bool {
        if self.len() < 2 {
            return false;
        }
        let mut i = self.len() - 1;
        while i > 0 && self[i - 1] >= self[i] {
            i -= 1;
        }
        if i == 0 {
            return false;
        }
        let mut j = self.len() - 1;
        while j >= i && self[j] <= self[i - 1] {
            j -= 1;
        }
        self.swap(j, i - 1);
        self[i..].reverse();
        true
    }

    fn prev_permutation(&mut self) -> bool {
        if self.len() < 2 {
            return false;
        }
        let mut i = self.len() - 1;
        while i > 0 && self[i - 1] <= self[i] {
            i -= 1;
        }
        if i == 0 {
            return false;
        }
        self[i..].reverse();
        let mut j = self.len() - 1;
        while j >= i && self[j - 1] < self[i - 1] {
            j -= 1;
        }
        self.swap(i - 1, j);
        true
    }
}

fn dfs(adjacency: &Vec<Vec<usize>>, result: &mut HashMap<usize, i64>, u: usize, d: i64) {
    if result.contains_key(&u) {
        return
    }
    result.insert(u, d);
    for &v in adjacency[u].iter() {
        dfs(adjacency, result, v, d + 1);
    }
}

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n - 1],
    }
    let mut adjacency = vec![vec![]; n + 1];
    for (ai, bi) in ab {
        adjacency[ai].push(bi);
        adjacency[bi].push(ai);
    }
    let mut from_1 = HashMap::new();
    dfs(&adjacency, &mut from_1, 1, 0);
    // println!("{:?}", from_1);
    let mut from_n = HashMap::new();
    dfs(&adjacency, &mut from_n, n, 0);
    // println!("{:?}", from_n);

    let mut count_1 = 0;
    let mut count_n = 0;
    for i in 1..n + 1 {
        if from_1[&i] <= from_n[&i] {
            count_1 += 1;
        } else {
            count_n += 1;
        }
    }
    if count_1 > count_n {
        println!("Fennec");
    } else {
        println!("Snuke");
    }
}
