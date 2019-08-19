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
        k: usize,
        v: [i32; n],
    }
    let mut result = 0;
    for i in 1..k+1 {
        let r = std::cmp::min(n, k - i);
        if i >= n {
            let mut nums = v.iter().map(|&vi| vi).collect::<Vec<_>>();
            nums.sort();
            let mut sum = nums.iter().sum();
            for j in 0..r {
                sum -= nums[j];
            }
            if sum > result {
                result = sum;
            }
            continue;
        }
        for left in 0..i+1 {
            let mut nums = Vec::with_capacity(i);
            let right = n - (i - left);
            for j in 0..left {
                nums.push(v[j]);
            }
            for j in right..n {
                nums.push(v[j]);
            }
            nums.sort();
            let mut sum = nums.iter().sum();
            for j in 0..nums.len() {
                if nums[j] > 0 || j >= r {
                    break;
                }
                sum -= nums[j];
            }
            if sum > result {
                result = sum;
            }
        }
    }
    println!("{}", result);
}
