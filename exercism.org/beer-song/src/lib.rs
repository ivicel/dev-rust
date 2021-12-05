pub fn verse(n: u32) -> String {
    get(n).or_else(|| Some(String::new())).unwrap()
}

pub fn sing(start: u32, end: u32) -> String {
    let mut is_first_line = true;
    let mut s = String::new();
    for i in (end..=start).rev() {
        if is_first_line {
            is_first_line = false;
        } else {
            s.push('\n');
        }
        s.push_str(&verse(i));
    }

    s
}

fn get(n: u32) -> Option<String> {
    match n {
        0 => Some(String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n")),
        i @ 1 ..=99 => {
            if i == 1 {
                Some(format!("{} bottle of beer on the wall, {} bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n", i, i))
            } else {
                let s = if i == 2 {
                    ""
                } else {
                    "s"
                };
                Some(format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottle{} of beer on the wall.\n", i, i, i - 1, s))
            }
        },
        _ => None
    }
}
