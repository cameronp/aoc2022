// turn off dead code warning
#![allow(dead_code)]


#[derive(Debug)]
enum  List {
    Cons(i32, Rc<List>),
    Nil,
}

fn head(list: &List) -> Option<i32> {
    match list {
        List::Cons(head, _) => Some(*head),
        List::Nil => None,
    }
}

fn tail(list: &List) -> Option<&List> {
    match list {
        List::Cons(_, tail) => Some(tail),
        List::Nil => None,
    }
}

use std::rc::Rc;
use std::ops::Deref;
struct MyBox<T>(T);
impl <T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T>{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

use crate::List::{Cons, Nil};
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}