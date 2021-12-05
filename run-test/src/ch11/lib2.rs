pub trait MyTrait {
    fn show(&self);
}

impl MyTrait for i32 {
    fn show(&self) {
        println!("MyTrait: {:?}", self);
    }
}

mod test {
    #![allow(unused_imports)]
    use super::MyTrait;

    #[test]
    fn test1() {
        let i = 10i32;
        i.show();
        let zero = -10;
        println!("{}", i64::abs(zero));
    }
}
