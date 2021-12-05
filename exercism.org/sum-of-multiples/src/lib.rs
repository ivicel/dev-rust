use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut res = HashSet::new();
    for &i in factors {
        if i == 0 || i > limit {
            continue;
        }

        for n in i..limit {
            if n % i == 0 {
                res.insert(n);
            }
        }
    }

    res.iter().sum::<u32>()
}
