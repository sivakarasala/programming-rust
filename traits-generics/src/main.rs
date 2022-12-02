use std::io::Write;
use std::fs::File;

// plain function with trait object as an argument
// fn say_hello(out: &mut dyn Write) -> std::io::Result<()> {
//     out.write_all(b"hello world\n")?;
//     out.flush()
// }

// generic fns with type parameter
fn say_hello<W: Write>(out: &mut W) -> std::io::Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
}

fn min<T: Ord>(value1: T, value2: T) -> T {
    if value1 <= value2 {
        value1
    } else {
        value2
    }
}

// A trait is a feature that any given type may or may not support.
// Most often, a trait represents a capability: something a type can do.

fn main() {
    // let mut local_file = File::create("hello.txt").unwrap();
    // say_hello(&mut local_file).unwrap();

    // let mut bytes = vec![];
    // say_hello(&mut bytes).unwrap();
    // assert_eq!(bytes, b"hello world\n");

    let mut buf: Vec<u8> = vec![];
    // A reference to  a trait type, like `writer`, is called a trait object.
    // Like any other reference, a trait object points to some value, it has 
    // a lifetime, and it can be either mut or shared.
    let writer: &mut dyn Write = &mut buf;

    // let w: Box<dyn Write> = Box::new(local_file);
    // say_hello::<File>(&mut local_file)?;

    // calling a generic method collect<C>() that takes no arguments
    // let v1 = (0 .. 1000).collect(); // error: can't infer type
    let v2 = (0 .. 1000).collect::<Vec<i32>>(); // ok
}
// defining a trait
// give it a name and list the type signatures of the trait methods
trait Visible {
    fn draw(&self, canvas: &mut Canvas);

    fn hit_test(&self, x: i32, y: i32) -> bool;
}

// to implement a trait, use the syntax `impl TraitName for Type`
impl Visible for Broom {
    fn draw(&self, canvas: &mut Canvas) {
        for y in self.y - self.height - 1 .. self.y {
            canvas.write_at(self.x, y, '|');
        }
        canvas.write_at(self.x, self.y, 'M');
    }

    fn hit_test(&self, x: i32, y: i32) -> bool {
        self.x == x
        && self.y - self.height - 1 <= y
        && y <= self.y
    }
}

pub struct Sink;

use std::io::{Write, Result};

impl Write for Sink {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

let mut out = Sink;
out.write_all(b"hello world\n")?;

// Default methods
trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        let mut bytes_written = 0;
        while bytes_written < buf.len() {
            bytes_written += self.write(&buf[bytes_written..])?;
        }
        Ok(())
    }
}

// Subtraits
// Every type that implements `Creature` must also implement the
// `Visible` trait
trait Creature: Visible {
    fn position(&self) -> (i32, i32);
    fn facing(&self) -> Direction;
}

// type associated functions
// traits can include type-associated fns, like static methods

trait StringSet {
    //return a new empty set.
    fn new() -> Self;

    // return a set that contains all the strings in `strings`.
    fn from_slice(strings: &[&str]) -> Self;

    // find out if this set contains a particalar `value`.
    fn contains(&self, string: &str) -> bool;

    // add a string to this set.
    fn add(&mut self, string: &str);
}

// trait objects to support type-associated funcs. 
// change the trait, adding the bound `where Self: Sized` to each
// associated fn that doesn't take a `self` arg by ref
// this bound tells rust that trait objects are excused from supporting
// this particular associated fn.

trait StringSet {
    fn new() -> Self
        where Self: Sized;

    fn from_slice(strings: &[&str]) -> Self
        where Self: Sized;

    fn contains(&self, string: &str) -> bool;

    fn add(&mut self, string: &str);
}

// fully qualified method calls
"hello".to_string();
str::to_string("hello");
ToString::to_string("hello");
<str as ToString>::to_string("hello");

// associated consts
trait Greet {
    const GREETING: &'static str = "Hello";
    fn greet(&self) -> String;
}

trait Float {
    const ZERO: Self;
    const ONE: Self;
}

impl Float for f32 {
    const ZERO: f32 = 0.0;
    const ONE: f32 = 1.0;
}

impl Float for f64 {
    const ZERO: f64 = 0.0;
    const ONE: f64 = 1.0;
}