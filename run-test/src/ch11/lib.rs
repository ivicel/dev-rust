#![allow(dead_code)]

use std::io::{Result, Write};

pub fn say_hello(out: &mut dyn Write) -> Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
}

/// generic function
pub fn say_hello2<W: Write>(w: &mut W) -> Result<()> {
    w.write_all(b"hello world\n")?;
    w.flush()
}

#[derive(Debug, PartialEq, Eq)]
pub struct Foo;

pub trait Bar {
    fn baz(&self);
}

impl Bar for Foo {
    fn baz(&self) {
        println!("{:?}", self);
    }
}

pub fn static_dispatch<T: Bar>(t: &T) {
    t.baz();
}

pub fn dynamic_dispatch(t: &dyn Bar) {
    t.baz();
}

mod test {
    #![allow(unused_imports)]
    use std::{
        fs::File,
        io::{Result, Write},
    };

    use super::{Foo, dynamic_dispatch, say_hello, static_dispatch};

    #[test]
    fn test1() -> Result<()> {
        let mut local_file = File::create("hello.txt")?;

        let l = File::create("hello2.txt")?;
        // trait object
        let _w: Box<dyn Write> = Box::new(&l);

        say_hello(&mut local_file)
    }

    #[test]
    fn test2() {
        let v1 = Foo;
        static_dispatch(&v1);
        dynamic_dispatch(&v1);
    }
}
