use std::collections::HashMap;

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

fn main() {
    input! {
        n: usize,
        v: [usize; n],
    }
    let mut odd_nums = HashMap::new();
    let mut even_nums = HashMap::new();
    for i in 0..n {
        if i % 2 == 0 {
            *even_nums.entry(v[i]).or_insert(0) += 1
        } else {
            *odd_nums.entry(v[i]).or_insert(0) += 1
        }
    }
    let mut odd_nums = odd_nums.iter().collect::<Vec<_>>();
    odd_nums.sort_by_key(|&(_, count)| count);
    odd_nums.reverse();
    let mut even_nums = even_nums.iter().collect::<Vec<_>>();
    even_nums.sort_by_key(|&(_, count)| count);
    even_nums.reverse();

    if odd_nums[0].0 == even_nums[0].0 {
        if odd_nums.len() == 1 {
            println!("{}", n - even_nums[0].1);
            return;
        } else if even_nums.len() == 1 {
            println!("{}", n - odd_nums[0].1);
            return;
        } else if odd_nums[1].1 < even_nums[1].1 {
            println!("{}", n - odd_nums[0].1 - even_nums[1].1);
        } else {
            println!("{}", n - even_nums[0].1 - odd_nums[1].1);
        }
    } else {
        println!("{}", n - odd_nums[0].1 - even_nums[0].1);
    }
}
