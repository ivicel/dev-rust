pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut res = Vec::new();

    if digits.len() < len {
        return res;
    }

    let s = digits.chars().collect::<Vec<char>>();
    for i in 0..=s.len() {
        if len == 0 {
            res.push(String::new());
            continue;
        }

        let string = s[i..i + len].iter().collect::<String>();
        res.push(string);

        if i + len == s.len() {
            break;
        }
    }

    res
}
