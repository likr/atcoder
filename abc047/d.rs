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
        t: usize,
        a: [usize; n],
    }

    let mut min = a[0];
    let mut max = a[0];
    let mut diff = 0;
    let mut result = 0;
    for i in 1..n {
        // println!("  {} {}", a[i], result);
        if a[i] < min {
            let new_diff = max - min;
            // println!("{} {}", diff, new_diff);
            if new_diff == diff {
                result += 1;
            } else if new_diff > diff {
                diff = new_diff;
                result = 1;
            }
            max = a[i];
            min = a[i];
        } else if a[i] > max {
            max = a[i];
        }
    }
    let new_diff = max - min;
    // println!("{} {}", min, max);
    // println!("{} {}", diff, new_diff);
    if new_diff == diff {
        result += 1;
    } else if new_diff > diff {
        result = 1;
    }
    
    println!("{}", result);
}
