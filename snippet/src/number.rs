use cargo_snippet::snippet;

#[snippet("factors")]
pub fn factors(n: usize) -> Vec<usize> {
    let mut result = vec![];
    for d in 1.. {
        if d * d > n {
            break;
        }
        if n % d == 0 {
            result.push(d);
            result.push(n / d);
        }
    }
    result.sort();
    result.dedup();
    result
}

#[snippet("prime_factors")]
pub fn prime_factors(n: usize) -> Vec<(usize, usize)> {
    let mut result = vec![];
    let mut m = n;
    for d in 2.. {
        if d * d > n {
            break;
        }
        let mut count = 0;
        while m % d == 0 {
            count += 1;
            m /= d;
        }
        if count > 0 {
            result.push((d, count));
        }
    }
    if m > 1 {
        result.push((m, 1));
    }
    result
}
