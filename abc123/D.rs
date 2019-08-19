#[allow(unused_macros)]
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

#[allow(unused_macros)]
macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };

    ($next:expr, mut $var:ident : $t:tt $($r:tt)*) => {
        let mut $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

#[allow(unused_macros)]
macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, [ $t:tt ]) => {
        {
            let len = read_value!($next, usize);
            (0..len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
        }
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, bytes) => {
        read_value!($next, String).into_bytes()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn select(v: &mut Vec<usize>, start: usize, stop: usize, k: usize) {
    println!("{} {}", start, stop);
    let m = (start + stop) / 2;
    v.swap(start, m);
    let mut left = start + 1;
    let mut right = stop - 1;
    while left <= right {
        if v[left] >= v[start] {
            left += 1;
        } else {
            v.swap(left, right);
            right -= 1;
        }
    }
    v.swap(start, left - 1);
    if left < k {
        select(v, left, stop, k);
    } else if k < left {
        select(v, start, left - 1, k);
    }
}

fn main() {
    input! {
        x: usize,
        y: usize,
        z: usize,
        k: usize,
        a: [usize; x],
        b: [usize; y],
        c: [usize; z],
    }
    let mut abc = Vec::with_capacity(x * y * z);
    for ai in a.iter() {
        for bi in b.iter() {
            for ci in c.iter() {
                abc.push(ai + bi + ci);
            }
        }
    }
    let n = abc.len();
    select(&mut abc, 0, n, k);
    let (first, _) = abc.split_at_mut(k);
    first.sort();
    first.reverse();
    for i in 0..k {
        println!("{}", first[i]);
    }
}
