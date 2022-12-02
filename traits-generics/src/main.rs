use std::io::Write;
use std::fs::File;

// plain function with trait object as an argument
fn say_hello(out: &mut dyn Write) -> std::io::Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
}

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
}
