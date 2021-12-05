use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut v = input.graphemes(true).collect::<Vec<&str>>();
    v.reverse();
    println!("{:?}", v);
    String::from_iter(v)
}
