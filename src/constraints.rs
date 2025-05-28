use std::iter::repeat_n;

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

    factors
}

fn factors(n: u64) -> Vec<u64> {
    let prime_factors = prime_factors(n);
    let mut factors = Vec::new();
    let mut selected: Vec<bool> = repeat_n(false, prime_factors.len()).collect();

    while selected.iter().any(|s| !s) {
        factors.push(
            prime_factors
                .iter()
                .zip(selected.iter())
                .fold(1u64, |acc, (p, s)| if *s { acc * p } else { acc }),
        );
        for s in selected.iter_mut() {
            if *s {
                *s = false;
            } else {
                *s = true;
                break;
            }
        }
    }

    factors.push(n);
    factors.sort();
    factors.dedup();
    factors
}

fn checked_sum(values: impl AsRef<[u64]>) -> Option<u64> {
    values
        .as_ref()
        .iter()
        .try_fold(0u64, |acc, v| acc.checked_add(*v))
}

fn checked_product(values: impl AsRef<[u64]>) -> Option<u64> {
    values
        .as_ref()
        .iter()
        .try_fold(1u64, |acc, v| acc.checked_mul(*v))
}

pub fn add_enumerator(target: u64, number_count: usize, number_max: u64) -> Vec<Vec<u64>> {
    let mut solutions = Vec::new();
    let mut current: Vec<u64> = std::iter::repeat_n(1, number_count).collect();

    'search: loop {
        if let Some(remainder) =
            checked_sum(&current[0..number_count - 1]).and_then(|s| target.checked_sub(s))
            && remainder <= number_max
        {
            current[number_count - 1] = remainder;
            solutions.push(current.clone());
        }
        for i in (0..number_count - 1).rev() {
            let n = current[i] + 1;
            current[i..].fill(n);
            if current[i] < number_max && checked_sum(&current).is_some_and(|s| s <= target) {
                break;
            }
            if i == 0 {
                break 'search;
            }
        }
    }

    solutions
}

pub fn sub_enumerator(target: u64, number_count: usize, number_max: u64) -> Vec<Vec<u64>> {
    let mut solutions = Vec::new();
    let mut current: Vec<u64> = std::iter::repeat_n(1, number_count).collect();

    'search: loop {
        if let Some(remainder) =
            checked_sum(&current[0..number_count - 1]).and_then(|s| s.checked_add(target))
            && remainder <= number_max
        {
            current[number_count - 1] = remainder;
            solutions.push(current.clone());
        }
        for i in (0..number_count - 1).rev() {
            let n = current[i] + 1;
            current[i..].fill(n);
            if current[i] < number_max
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

    solutions
}

pub fn mul_enumerator(target: u64, number_count: usize, number_max: u64) -> Vec<Vec<u64>> {
    let factors = factors(target);
    let factor_upper_bound = factors
        .iter()
        .enumerate()
        .find(|(_, f)| **f > number_max)
        .map(|(i, _)| i)
        .unwrap_or(factors.len());
    let mut solutions = Vec::new();
    let mut current = Vec::with_capacity(number_count - 1);
    let mut remainder = target;
    current.push(0);

    while !current.is_empty() {
        let depth = current.len();
        let mut backtrack = true;
        // Adjust last idx of current so that current forms a solution prefix
        for (i, f) in factors
            .iter()
            .enumerate()
            .take(factor_upper_bound)
            .skip(current[depth - 1])
        {
            // Factor pow check is meant to prune impossible prefixes
            if remainder % f == 0
                && f.checked_pow((number_count + 1 - depth) as u32)
                    .is_some_and(|p| p <= remainder)
            {
                remainder /= f;
                current[depth - 1] = i;
                backtrack = false;
                break;
            }
        }
        if backtrack {
            // Backtrack if it failed to find a solution prefix
            current.pop();
            if let Some(prev_idx) = current.last_mut() {
                remainder *= factors[*prev_idx];
                *prev_idx += 1;
            }
        } else if current.len() < number_count - 1 {
            // If we have a valid but incomplete solution prefix, we extend by one number
            current.push(current[depth - 1]);
        } else if current.len() == number_count - 1 {
            // If our prefix has reached number_count - 1, we check if it forms a valid solution with the remainder
            if factors[current[depth - 1]] <= remainder && remainder <= number_max {
                solutions.push(
                    current
                        .iter()
                        .map(|i| factors[*i])
                        .chain([remainder])
                        .collect(),
                );
            }
            // we start over the search from the last index of current
            remainder *= factors[current[depth - 1]];
            current[depth - 1] += 1;
        }
    }

    solutions
}

pub fn div_enumerator(target: u64, number_count: usize, number_max: u64) -> Vec<Vec<u64>> {
    let mut solutions = Vec::new();
    let mut current: Vec<u64> = std::iter::repeat_n(1, number_count).collect();

    'search: loop {
        if let Some(remainder) =
            checked_product(&current[0..number_count - 1]).and_then(|p| p.checked_mul(target))
            && remainder <= number_max
        {
            current[number_count - 1] = remainder;
            solutions.push(current.clone());
        }
        for i in (0..number_count - 1).rev() {
            let n = current[i] + 1;
            current[i..].fill(n);
            if current[i] < number_max
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

    solutions
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
            vec![vec![1, 2, 6], vec![1, 3, 4], vec![2, 2, 3]]
        );
        assert_eq!(
            mul_enumerator(30, 3, 10),
            vec![vec![1, 3, 10], vec![1, 5, 6], vec![2, 3, 5]]
        );
        assert_eq!(
            mul_enumerator(45, 3, 10),
            vec![vec![1, 5, 9], vec![3, 3, 5]]
        );
        assert_eq!(
            mul_enumerator(60, 4, 10),
            vec![
                [1, 1, 6, 10],
                [1, 2, 3, 10],
                [1, 2, 5, 6],
                [1, 3, 4, 5],
                [2, 2, 3, 5]
            ]
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
