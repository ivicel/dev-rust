#![allow(dead_code)]

use std::{convert::TryInto, fmt::Debug, ops::Rem};

trait Add<Rhs = Self, Output = Self> {
    fn add(self, other: Rhs) -> Output;
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    fn add(self, other: Self) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + self.y,
        }
    }
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Add<Point, Line> for Point {
    fn add(self, other: Point) -> Line {
        Line {
            start: self,
            end: other,
        }
    }
}

mod test {
    #![allow(unused_imports)]

    use std::{
        fs::File,
        io::{Read, Result},
    };

    use super::{Add, Line, Point};

    #[test]
    fn test1() {
        let a = Point { x: 3, y: 2 };
        let b = Point { x: 1, y: 2 };
        let c: Line = a.add(b);
        println!("{:?}", c);
    }

    #[test]
    fn test2() -> Result<()> {
        let mut file = File::open("hello.txt")?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        Ok(())
    }
}

trait Greet {
    fn greet(&self, name: &str) -> String;
    fn greet_loundly(&self, name: &str) -> String {
        self.greet(name) + "!"
    }
}

struct Hello;
struct Hola;

impl Greet for Hello {
    fn greet(&self, name: &str) -> String {
        format!("Hello {}", name)
    }
}

impl Greet for Hola {
    fn greet(&self, name: &str) -> String {
        format!("Hola {}", name)
    }

    fn greet_loundly(&self, name: &str) -> String {
        let mut greeting = self.greet(name);
        greeting.insert(0, 'i');
        greeting + "!"
    }
}

trait Even {
    fn is_even(self) -> bool;
}

// impl Even for i32 {
//     fn is_even(self) -> bool {
//         todo!()
//     }
// }

// impl Even for i64 {
//     fn is_even(self) -> bool {
//         todo!()
//     }
// }

impl<T> Even for T
where
    T: Rem<Output = T> + PartialEq + Sized,
    u8: TryInto<T>,
    <u8 as TryInto<T>>::Error: Debug,
{
    fn is_even(self) -> bool {
        self % 2.try_into().unwrap() == 0.try_into().unwrap()
    }
}

mod test2 {
    #![allow(unused_imports)]

    #[test]
    fn test1() {}
}

trait Wizard {
    fn fly(&self);
}

trait Pilot {
    fn fly(&self);
}

struct Human;

impl Wizard for Human {
    fn fly(&self) {
        println!("human wizard...");
    }
}

impl Pilot for Human {
    fn fly(&self) {
        println!("human pilot...")
    }
}

impl Human {
    fn fly(&self) {
        println!("human...")
    }
}

mod test3 {
    #![allow(unused_imports)]

    use super::{Human, Wizard};

    #[test]
    fn test1() {
        let h = Human;
        Wizard::fly(&h);
        h.fly();
    }
}

trait SuperTrait {
    fn f1(&self) {
        println!("in super trait.")
    }
}

trait SubTrait: SuperTrait {
    fn f1(&self) {
        println!("in sub trait.")
    }
}

struct SomeType;

impl SuperTrait for SomeType {}

impl SubTrait for SomeType {}

trait SuperTrait2 {
    fn super_method(&mut self);
}

trait SubTrait2: SuperTrait2 {
    fn sub_method(&mut self);
}

struct CallSuperFromSub;

impl SuperTrait2 for CallSuperFromSub {
    fn super_method(&mut self) {
        println!("in super");
    }
}

impl SubTrait2 for CallSuperFromSub {
    fn sub_method(&mut self) {
        println!("in sub");
        self.super_method();
    }
}

struct CallSubFromSuper;

impl SuperTrait2 for CallSubFromSuper {
    fn super_method(&mut self) {
        println!("in super");
        self.sub_method();
    }
}

impl SubTrait2 for CallSubFromSuper {
    fn sub_method(&mut self) {
        println!("in sub");
    }
}

struct CallEachOther(bool);

impl SuperTrait2 for CallEachOther {
    fn super_method(&mut self) {
        println!("in super");
        if self.0 {
            self.0 = false;
            self.sub_method();
        }
    }
}

impl SubTrait2 for CallEachOther {
    fn sub_method(&mut self) {
        println!("in sub");
        if self.0 {
            self.0 = false;
            self.super_method();
        }
    }
}

mod test4 {
    #![allow(unused_imports)]
    use super::{
        CallEachOther, CallSubFromSuper, CallSuperFromSub, SomeType, SubTrait, SubTrait2,
        SuperTrait, SuperTrait2,
    };

    #[test]
    fn test1() {
        let s = SomeType;
        SubTrait::f1(&s);
        <SomeType as SuperTrait>::f1(&s);

        println!("------------");
        let mut s2 = CallSuperFromSub;
        s2.sub_method();

        println!("------------");
        let mut s3 = CallSubFromSuper;
        s3.super_method();

        println!("------------");
        let mut s4 = CallEachOther(true);
        s4.super_method();
    }
}

mod test5 {

    fn get_max_number<T>(array: &[T]) -> T
    where
        T: Copy + PartialOrd,
    {
        let mut result = array[0];
        for &x in array {
            if x > result {
                result = x;
            }
        }

        result
    }

    #[test]
    fn test1() {
        println!("Hello, lgli!");
        let array: [i8; 6] = [12, 2, 31, 4, 2, 6];
        let result = get_max_number(&array);
        println!("i8 数组中最大值是: {}", result);

        let array: [i32; 6] = [1222, 3, 5, 2, 9222, 10];
        let result = get_max_number(&array);
        println!("i32 数组中最大值是: {}", result);
    }

    #[test]
    fn test2() {
        // let x: Option<String> = Some(String::from("hello, world"));
        let x: Option<String> = None;
        if let Some(ref s) = x {
            println!("{}", s)
        }

        if let Some(s) = x.as_ref() {
            println!("got string: {}", s);
        }

        println!("{:?}", x);
    }
}
