pub fn nth(n: u32) -> u32 {
    let mut count = 0;
    for i in 2u32.. {
        if is_prime(i) {
            count += 1;
            if count == n + 1 {
                count = i;
                break;
            }
        }
    }

    count
}

fn is_prime(n: u32) -> bool {
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }

    true
}
