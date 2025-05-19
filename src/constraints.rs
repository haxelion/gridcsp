use std::iter::repeat;

fn prime_factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut m = 2u64;

    while n >= m * m {
        while n % m == 0 {
            n /= m;
            factors.push(m);
        }
        m += 1;
    }

    if n != 1 {
        factors.push(n);
    }

    return factors;
}

fn checked_sum(values: impl AsRef<[u64]>) -> Option<u64> {
    values
        .as_ref()
        .iter()
        .fold(Some(0u64), |acc, v| acc.and_then(|a| a.checked_add(*v)))
}

fn checked_product(values: impl AsRef<[u64]>) -> Option<u64> {
    values
        .as_ref()
        .iter()
        .fold(Some(1u64), |acc, v| acc.and_then(|a| a.checked_mul(*v)))
}

pub fn add_enumerator(target: u64, number_count: usize, number_max: u64) -> Vec<Vec<u64>> {
    let mut solutions = Vec::new();
    let mut current: Vec<u64> = repeat(1).take(number_count).collect();

    'search: loop {
        if let Some(remainder) =
            checked_sum(&current[0..number_count - 1]).and_then(|s| target.checked_sub(s))
        {
            if remainder <= number_max {
                current[number_count - 1] = remainder;
                solutions.push(current.clone());
            }
        }
        for i in (0..number_count - 1).rev() {
            let n = current[i] + 1;
            current[i..].fill(n);
            if current[i] + 1 <= number_max && checked_sum(&current).is_some_and(|s| s <= target) {
                break;
            }
            if i == 0 {
                break 'search;
            }
        }
    }

    return solutions;
}

pub fn sub_enumerator(target: u64, number_count: usize, number_max: u64) -> Vec<Vec<u64>> {
    let mut solutions = Vec::new();
    let mut current: Vec<u64> = repeat(1).take(number_count).collect();

    'search: loop {
        if let Some(remainder) =
            checked_sum(&current[0..number_count - 1]).and_then(|s| s.checked_add(target))
        {
            if remainder <= number_max {
                current[number_count - 1] = remainder;
                solutions.push(current.clone());
            }
        }
        for i in (0..number_count - 1).rev() {
            let n = current[i] + 1;
            current[i..].fill(n);
            if current[i] + 1 <= number_max
                && checked_sum(&current[0..number_count - 1])
                    .and_then(|s| s.checked_add(target))
                    .is_some_and(|s| s <= number_max)
            {
                break;
            }
            if i == 0 {
                break 'search;
            }
        }
    }

    return solutions;
}

pub fn mul_enumerator(target: u64, number_count: usize, number_max: u64) -> Vec<Vec<u64>> {
    let mut solutions = Vec::new();
    let factors = prime_factors(target);
    let mut indexes: Vec<usize> = repeat(0).take(factors.len()).collect();

    'search: loop {
        let mut solution: Vec<u64> = repeat(1).take(number_count).collect();
        for i in 0..indexes.len() {
            solution[indexes[i]] *= factors[i];
        }
        if solution.iter().all(|s| *s <= number_max) {
            solution.sort();
            if !solutions.contains(&solution) {
                solutions.push(solution);
            }
        }
        for i in (0..indexes.len()).rev() {
            if indexes[i] + 1 < number_count {
                indexes[i] += 1;
                indexes[i + 1..].fill(0);
                break;
            }
            if i == 0 {
                break 'search;
            }
        }
    }

    return solutions;
}

pub fn div_enumerator(target: u64, number_count: usize, number_max: u64) -> Vec<Vec<u64>> {
    let mut solutions = Vec::new();
    let mut current: Vec<u64> = repeat(1).take(number_count).collect();

    'search: loop {
        if let Some(remainder) =
            checked_product(&current[0..number_count - 1]).and_then(|p| p.checked_mul(target))
        {
            if remainder <= number_max {
                current[number_count - 1] = remainder;
                solutions.push(current.clone());
            }
        }
        for i in (0..number_count - 1).rev() {
            let n = current[i] + 1;
            current[i..].fill(n);
            if current[i] + 1 <= number_max
                && checked_product(&current[0..number_count - 1])
                    .and_then(|p| p.checked_mul(target))
                    .is_some_and(|p| p <= number_max)
            {
                break;
            }
            if i == 0 {
                break 'search;
            }
        }
    }

    return solutions;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prime_factors_test() {
        assert_eq!(prime_factors(5), vec![5]);
        assert_eq!(prime_factors(12), vec![2, 2, 3]);
        assert_eq!(prime_factors(60), vec![2, 2, 3, 5]);
        assert_eq!(prime_factors(60), vec![2, 2, 3, 5]);
        assert_eq!(prime_factors(93), vec![3, 31]);
    }

    #[test]
    fn add_enumerator_test() {
        assert_eq!(add_enumerator(3, 2, 3), vec![vec![1, 2]]);
        assert_eq!(add_enumerator(4, 2, 3), vec![vec![1, 3], vec![2, 2]]);
        assert_eq!(add_enumerator(4, 3, 3), vec![vec![1, 1, 2]]);
        assert_eq!(add_enumerator(5, 3, 3), vec![vec![1, 1, 3], vec![1, 2, 2]]);
        assert_eq!(add_enumerator(6, 3, 3), vec![vec![1, 2, 3], vec![2, 2, 2]]);
        assert_eq!(
            add_enumerator(6, 3, 4),
            vec![vec![1, 1, 4], vec![1, 2, 3], vec![2, 2, 2]]
        );
    }

    #[test]
    fn sub_enumerator_test() {
        assert_eq!(
            sub_enumerator(3, 2, 6),
            vec![vec![1, 4], vec![2, 5], vec![3, 6]]
        );
        assert_eq!(sub_enumerator(4, 3, 6), vec![vec![1, 1, 6]]);
        assert_eq!(sub_enumerator(3, 3, 6), vec![vec![1, 1, 5], vec![1, 2, 6]]);
    }

    #[test]
    fn mul_enumerator_test() {
        assert_eq!(mul_enumerator(3, 2, 3), vec![vec![1, 3]]);
        assert_eq!(
            mul_enumerator(12, 3, 6),
            vec![vec![1, 3, 4], vec![1, 2, 6], vec![2, 2, 3]]
        );
        assert_eq!(
            mul_enumerator(30, 3, 10),
            vec![vec![1, 5, 6], vec![1, 3, 10], vec![2, 3, 5]]
        );
    }

    #[test]
    fn div_enumerator_test() {
        assert_eq!(div_enumerator(3, 2, 6), vec![vec![1, 3], vec![2, 6]]);
        assert_eq!(
            div_enumerator(2, 3, 6),
            vec![vec![1, 1, 2], vec![1, 2, 4], vec![1, 3, 6]]
        );
        assert_eq!(
            div_enumerator(3, 3, 10),
            vec![vec![1, 1, 3], vec![1, 2, 6], vec![1, 3, 9]]
        );
    }
}
