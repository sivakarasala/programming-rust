// Readers and Writers

// Read
//      Stdin, File, TcpStream

// BufRead
//       All of Read, BufReader<R>, Cursor<&[u8]>, StdinLock

// Write
//      Stdout, Stderr, File, TcpStream, Vec<u8>, BurWriter<W>

// Readers
//      Are values that any program can read bytes from.
//  Examples:
//  Files opened using std::fs::File::open(filename)
// std::net::TcpStream s for receiving data over the network
// std::io::stdin(), for reading from the process's standard input stream
// std::io::Cursor<&[u8]> and std::io::Cursor<Vec<u8>> values, which are
// readers that "read" from a byte array or vector that's already in memory

// Writers
//      Are values that any program can write bytes to.
// Examples:
// Files opened using std::fs::File::create(filename)
// std::net::TcpStream s, for sending data over the network
// std::io::stdout() and std::io::stderr(), for writing to the terminal
// Vec<u8>, a writer whose write methods append to the vector
// std::io::Cursor<Vec<u8>>, which is similar but lets you both read and write
// data, and seek to different positions within the vector

use std::io::{self, Read, Write, ErrorKind};

const DEFAULT_BUF_SIZE: usize = 8 * 1024;

pub fn copy<R: ?Sized, W: ?Sized>(reader: &mut R, writer: &mut W) -> io::Result<u64>
    where R: Read, W: Write
{
    let mut buf = [0; DEFAULT_BUF_SIZE];
    let mut written = 0;
    loop {
        let len = match reader.read(&mut buf) {
            Ok(0) => return Ok(written),
            Ok(len) => len,
            Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
            Err(e) => return Err(e), 
        };
        writer.write_all(&buf[..len])?;
        written += len as u64;
    }
}

// reader.read(&mut buffer) -> io::Result<u64>
// reader.read_to_end(&mut byte_vec) -> io::Result<usize>
// reader.read_to_string(&mut string)
// reader.read_exact(&mut buf)
// adapter methods
// reader.bytes() -> io::Result<u8>
// reader.chain(reader2)
// reader.take(n)

fn main() {
    println!("Hello, world!");
}
