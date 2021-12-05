#![allow(dead_code)]

enum List {
    Empty,
    Elem(i32, Box<List>),
}
