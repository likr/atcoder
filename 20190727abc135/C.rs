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

fn main() {
    input! {
        n: usize,
        a: [usize; n + 1],
        b: [usize; n],
    }
    let mut x = a.iter().map(|&ai| ai).collect::<Vec<_>>();
    let mut left_count = 0;
    for i in 0..n {
        if b[i] >= x[i] {
            let r = b[i] - x[i];
            left_count += x[i];
            x[i] = 0;
            x[i + 1] = if r >= x[i + 1] {
                left_count += x[i + 1];
                0
            } else {
                left_count += r;
                x[i + 1] - r
            }
        } else {
            left_count += b[i];
            x[i] -= b[i];
        }
    }
    let mut y = a.iter().map(|&ai| ai).collect::<Vec<_>>();
    let mut right_count = 0;
    for i in (1..n + 1).rev() {
        if b[i - 1] >= y[i] {
            let r = b[i - 1] - y[i];
            right_count += y[i];
            y[i] = 0;
            y[i - 1] = if r >= y[i - 1] {
                right_count += y[i - 1];
                0
            } else {
                right_count += r;
                y[i - 1] - r
            }
        } else {
            right_count += b[i - 1];
            y[i] -= b[i - 1];
        }
    }
    if left_count > right_count {
        println!("{}", left_count);
    } else {
        println!("{}", right_count);
    }
}
