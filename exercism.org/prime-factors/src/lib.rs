pub fn factors(n: u64) -> Vec<u64> {
    let mut v: Vec<u64> = vec![];

    if n < 2 {
        return v;
    } else if n == 2 {
        v.push(2);
        return v;
    }

    let mut t = n;

    loop {
        let mut end = true;

        for i in 2..=t {
            if t % i == 0 {
                v.push(i);
                t = t / i;
                end = false;
                break;
            }
        }

        if end {
            return v;
        }
    }
}
