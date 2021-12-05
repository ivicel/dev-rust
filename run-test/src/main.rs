mod ch11;
mod linklist;
mod network;
mod test1;
mod test2;
mod test3;

fn main() {
    let v = vec![1u64, 2, 3, 4];
    let s: u64 = v.iter().sum();
    println!("{:?}", s);
}
