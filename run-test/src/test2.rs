#![allow(dead_code)]
use std::ops::Add;

#[derive(Debug)]
struct Person {
    name: String,
}

impl Add for Person {
    type Output = Person;

    fn add(self, rhs: Self) -> Self::Output {
        Person {
            name: self.name.add(&rhs.name),
        }
    }
}

mod test1 {
    #![allow(unused_imports)]
    use super::Person;

    #[test]
    fn test1() {
        let p1 = Person {
            name: String::from("bob"),
        };
        let p2 = Person {
            name: String::from("1243"),
        };

        let p3 = p1 + p2;
        println!("p3 = {:?}", p3);
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
struct Complex<T> {
    re: T,
    im: T,
}

impl<T> Complex<T> {
    fn new(re: T, im: T) -> Self {
        Complex { re, im }
    }
}

impl<T: Add<T, Output = T>> Add for Complex<T> {
    type Output = Complex<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

#[derive(Debug)]
struct Foo;

fn take_the_n(n: &mut u8) {
    *n += 2;
}

fn take_the_s(s: &mut String) {
    s.push_str("ing");
}

#[cfg(test)]
mod test2 {

    use crate::test2::{take_the_n, take_the_s};

    use super::Complex;

    #[test]
    fn complex_basics() {
        let first = Complex::new(3, 5);
        let second: Complex<i32> = Complex::default();
        let result = first + second;
        assert_eq!(result, first);

        let mut a = String::from("Owned string");
        let a_ref = &mut a;
        a_ref.push('!');
        println!("{}", a);

        let mut n = 5;
        let mut s = String::from("Borrow");
        take_the_n(&mut n);
        take_the_s(&mut s);
        println!("n = {}, s = {}", n, s);
    }
}

#[derive(Debug)]
enum Food {
    Cake,
    Pizza,
    Salad,
}

#[derive(Debug)]
struct Bag {
    food: Food,
}

#[derive(Debug)]
struct SomeRef<'a, T> {
    post: &'a T,
}

#[derive(Debug)]
struct Number<'a> {
    num: &'a u8,
}

impl<'a> Number<'a> {
    fn get_num(&self) -> &'a u8 {
        self.num
    }

    fn set_num(&mut self, new_number: &'a u8) {
        self.num = new_number
    }
}

enum Level {
    Error,
    Debug,
}

struct Logger<'a>(&'a str, Level);

fn configure_logger<'a, T>(_t: T)
where
    T: Send + 'static,
{
}

#[cfg(test)]
mod test3 {
    use crate::test2::{configure_logger, Level, Logger, SomeRef};

    use super::{Bag, Food};

    #[test]
    fn test1() {
        let bag = Bag { food: Food::Pizza };
        match bag.food {
            Food::Cake => println!("I got cake"),
            ref a => println!("I got {:?}", a),
        }

        println!("{:?}", bag);

        let n = Bag { food: Food::Cake };
        let m = SomeRef { post: &n };
        println!("{:?}", m);

        let other = "abc";
        let logger = Logger(other, Level::Error);
        configure_logger(logger);
    }
}
