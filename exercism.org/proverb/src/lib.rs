pub fn build_proverb(list: &[&str]) -> String {
    if list.len() == 0 {
        return String::new();
    }

    let first = list[0];
    // For want of a nail the shoe was lost.
    // For want of a shoe the horse was lost.
    // For want of a horse the rider was lost.
    // For want of a rider the message was lost.
    // For want of a message the battle was lost.
    // For want of a battle the kingdom was lost.
    // And all for the want of a nail.

    let mut res = vec![];

    for (i, &s) in list.iter().enumerate() {
        if i == list.len() - 1 {
            res.push(format!("And all for the want of a {}.", first));
        } else {
            res.push(format!("For want of a {} the {} was lost.", s, list[i + 1]));
        }
    }

    res.join("\n")
}
