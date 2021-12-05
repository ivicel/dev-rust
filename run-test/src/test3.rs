#![allow(dead_code)]

use std::{cell::Cell, error::Error, fmt::Display, path::Path};

struct Point {
    x: u8,
    y: u8,
    cached_sum: Cell<Option<u8>>,
}

impl Point {
    fn sum(&self) -> u8 {
        match self.cached_sum.get() {
            Some(sum) => {
                println!("Got from cache: {}", sum);
                sum
            }
            None => {
                let new_sum = self.x + self.y;
                self.cached_sum.set(Some(new_sum));
                println!("Set cache: {}", new_sum);
                new_sum
            }
        }
    }
}

#[cfg(test)]
mod test1 {
    use std::cell::Cell;

    use super::Point;

    #[test]
    fn test1() {
        let p = Point {
            x: 8,
            y: 9,
            cached_sum: Cell::new(None),
        };
        println!("Summed result: {}", p.sum());
        println!("Summed result: {}", p.sum());

        let m = None::<i32>;
        let o = m.map(|x| x * 2);
        println!("{:?}", o);
    }
}

#[derive(Debug)]
enum ParseError {
    Malformed,
    Empty,
}

#[derive(Debug)]
struct ReadErr {
    child_err: Box<dyn Error>,
}

impl Display for ReadErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed reading todo file")
    }
}

impl Error for ReadErr {
    fn description(&self) -> &str {
        "Todolist read failed: "
    }

    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Todo list parsing failed")
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        "TodoList parse failed: "
    }

    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}

#[derive(Debug)]
struct TodoList {
    tasks: Vec<String>,
}

impl TodoList {
    fn get_todos<P>(path: P) -> Result<TodoList, Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        let read_todos = read_todos(path);
        let parsed_todos = parse_todos(&read_todos?)?;
        Ok(parsed_todos)
    }
}

fn read_todos<P>(path: P) -> Result<String, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let raw_todos = std::fs::read_to_string(path).map_err(|e| ReadErr {
        child_err: Box::new(e),
    })?;
    Ok(raw_todos)
}

fn parse_todos(todo_str: &str) -> Result<TodoList, Box<dyn Error>> {
    let mut tasks = Vec::new();
    for i in todo_str.lines() {
        tasks.push(i.to_string());
    }

    if tasks.is_empty() {
        Err(ParseError::Empty.into())
    } else {
        Ok(TodoList { tasks })
    }
}

#[cfg(test)]
mod test2 {
    use super::TodoList;

    #[allow(deprecated)]
    #[test]
    fn test1() {
        let todos = TodoList::get_todos("path/todo.txt");
        match todos {
            Ok(list) => println!("{:?}", list),
            Err(e) => {
                println!("{:?}", e);
                println!("{}", e.description())
            }
        }
    }
}

#[derive(Debug)]
enum Food {
    Pizza,
    Salad,
}

#[derive(Debug)]
enum PaymentMode {
    Bitcoin,
    Credit,
}

#[derive(Debug)]
struct Order {
    count: u8,
    item: Food,
    payment: PaymentMode,
}

// #[derive(Debug, Clone, Copy)]
#[derive(Debug)]
struct A(i32);

#[derive(Debug)]
struct Container {
    item_count: u32,
    a: A,
}

/// 这里的解构是会 move, 所以如果没有实现 Copy 的话会报错, 因为解构类型是可变引用 &mut
#[allow(unused_variables, unused_assignments)]
fn increment_item(
    Container {
        mut item_count,
        ref mut a,
    }: &mut Container,
) {
    item_count += 1;
    a.0 = 15;
}

fn calculate_cost(Container { item_count, a: _ }: &Container) -> u32 {
    let rate = 67;
    rate * item_count
}

struct Primes {
    limit: usize,
}

impl Primes {
    fn iter(&self) -> PrimesIter {
        PrimesIter {
            index: 2,
            computed: compute_primes(self.limit),
        }
    }

    fn new(limit: usize) -> Primes {
        Primes { limit }
    }
}

struct PrimesIter {
    index: usize,
    computed: Vec<bool>,
}

impl Iterator for PrimesIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.index += 1;
            if self.index >= self.computed.len() {
                return None;
            } else if self.computed[self.index] {
                return Some(self.index);
            } else {
                continue;
            }
        }
    }
}

fn compute_primes(limit: usize) -> Vec<bool> {
    let mut sieve = vec![true; limit];
    let mut m = 2;
    while m * m < limit {
        if sieve[m] {
            for i in (m * 2..limit).step_by(m) {
                sieve[i] = false;
            }
        }
        m += 1;
    }
    sieve
}

#[derive(Debug)]
struct BB {
    b: i32,
}

#[derive(Debug)]
struct AA {
    a: i32,
}

impl From<AA> for BB {
    fn from(aa: AA) -> Self {
        BB { b: -aa.a }
    }
}

trait Foo {
    fn foo(&self);

    fn foo1(&self);
}

fn generic(_: &dyn Foo) {
    unimplemented!()
}

trait Driver {
    fn driving(&self) {
        println!("trait Driver...");
    }
}

struct MyCar;

impl MyCar {
    fn driving(&self) {
        println!("MyCar driving...")
    }
}

impl Driver for MyCar {}

trait Position {}

struct Coordinates(f64, f64);

impl Position for Coordinates {}

#[cfg(test)]
mod test3 {

    use std::mem::size_of_val;
    use std::sync::Arc;
    use std::thread;

    use crate::test3::{calculate_cost, Coordinates, Driver, MyCar, Position, A};
    use crate::test3::{increment_item, Container, Food};

    use super::{Order, Primes, AA, BB};

    #[test]
    fn test1() {
        let mut food_order = Order {
            count: 3,
            item: Food::Pizza,
            payment: super::PaymentMode::Bitcoin,
        };

        let Order { count, payment, .. } = &mut food_order;
        println!("count = {:?}, payment = {:?}", count, payment);

        let mut container = Container {
            item_count: 10,
            a: A(10),
        };
        increment_item(&mut container);
        println!("after increment_item: {:?}", container);

        let total_cost = calculate_cost(&container);
        println!("Total cost: {}", total_cost);
    }

    #[test]
    fn test2() {
        for i in 0..20 {
            print!("{}, ", i);
        }

        println!("");

        let primes = Primes::new(100);
        for i in primes.iter() {
            print!("{}, ", i);
        }

        println!("");
    }

    #[test]
    fn test3() {
        let a = AA { a: 10 };
        println!("origin AA: {:?}", a);
        let b = BB::from(a);
        println!("BB::from(AA): {:?}", b);
        let a = AA { a: 10 };
        // let b: BB = a.into();
        // println!("AA::into -> BB: {:?}", b);
        println!("AA::into -> BB: {:?}", <AA as Into<BB>>::into(a));

        let my_car = MyCar;
        my_car.driving();
        <MyCar as Driver>::driving(&my_car);
    }

    #[test]
    fn test4() {
        let a = String::from("hello");
        let fn_ = || println!("call fn: {}", a);
        fn_();
        fn_();

        let val = Coordinates(1.0, 2.0);
        let ref_: &Coordinates = &val;
        let pos_ref: &dyn Position = &val as &dyn Position;

        println!("val: {}", size_of_val(&val));
        println!("ref_: {}", size_of_val(&ref_));
        println!("pos_ref: {}", size_of_val(&pos_ref));
    }

    #[test]
    fn test5() {
        let thr = thread::spawn(|| {
            println!("Thread!");
            2 //"Much concurrent, such now".to_string()
        });

        println!("Hello, main thread");
        let res = thr.join().expect("abc");
        println!("got result: {}", res);
    }

    #[test]
    fn test6() {
        let nums = Arc::new(vec![1, 2, 3, 4, 5]);
        let mut childs = vec![];
        for n in 0..5 {
            let ns = nums.clone();
            let thr = thread::spawn(move || println!("{}", ns[n]));
            childs.push(thr);
        }

        for thr in childs {
            thr.join().unwrap();
        }
    }
}

#[cfg(test)]
mod test4 {
    use std::{
        sync::{Arc, Mutex},
        thread, vec,
    };

    #[test]
    fn test1() {
        let m = Mutex::new(0);
        let c = thread::spawn(move || {
            *m.lock().unwrap() += 1;
            let updated = *m.lock().unwrap();
            updated
        });

        let updated = c.join().unwrap();
        println!("updated = {:?}", updated);
    }

    #[test]
    fn test2() {
        let vec = Arc::new(Mutex::new(vec![]));
        let mut childs = vec![];
        for i in 0..5 {
            let v = vec.clone();
            let t = thread::spawn(move || {
                let mut v = v.lock().unwrap();
                v.push(i);
            });

            childs.push(t);
        }

        for c in childs {
            c.join().unwrap();
        }

        println!("{:?}", vec);
    }
}
