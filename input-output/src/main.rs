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

use std::io;
use std::io::prelude::*;
use std::io::{BufReader, ErrorKind};
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;


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

// Buffered Readers
// These can hold a chunk of memory(a buffer) that holds some input or output data
// in memory. This saves on system calls as they are often slow to make mulitple
// calls.

// reader.read_line(&mut line)
// reader.lines()
// reader.read_until(stop_byte, &mut byte_vec)
// reader.split(stop_byte)

// grep - Search stdin or some files for lines matching a given string

fn grep<R>(target: &str, reader: R) -> io::Result<()> 
    where R: BufRead
{
    for line_result in reader.lines() {
        let line = line_result?;
        if line.contains(target) {
            println!("{}", line)
        }
    }
    Ok(())
}

fn grep_main() -> Result<(), Box<dyn Error>> {
    // Get the command-line arguments. The first argument is the
    // string to search for; the rest are filenames.
    let mut args = std::env::args().skip(1);
    let target = match args.next() {
        Some(s) => s,
        None => Err("usage: grep PATTERN FILE...")?
    };
    let files: Vec<PathBuf> = args.map(PathBuf::from).collect();

    if files.is_empty() {
        let stdin = io::stdin();
        grep(&target, stdin.lock())?;
    } else {
        for file in files {
            let f = File::open(file)?;
            grep(&target, BufReader::new(f))?;
        }
    }

    Ok(())
}

// collecting lines

// let results: Vec<io::Result<String>> = reader.lines().collect(); // ok, but not what we want

// let lines: Vec<String> = reader.lines().collect(); // error: can't convert Results to Vec<String>

// let mut lines = vec![];
// for line_result in reader.lines() {
//     lines.push(line_result?);
// }

// let lines = reader.lines().collect::<io::Result<Vec<String>>>()?;

fn main() {
    let result = grep_main();
    if let Err(err) = result {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

// Writers

// println!("Hello, world");
// println!("The greatest common divisor of {:?} is {}", numbers, d);
// println!();

// writeln!(io::stderr(), "error: world not helloable")?;
// writeln!(&mut byte_vec, "The greatest common divisor of {:?} is {}", numbers, d)?;

// write macros each take an extra first arg, a writer.
// The other is that they return a Result, so errors must be handled.

// Methods of Write trait
// writer.write(&buf)
// writer.write_all(&buf)
// writer.flush()

// let file = File::create("tmp.txt")?;
// let writer = BufWriter::new(file);
// let writer = BufWriter::with_capacity(size, writer);

use std::fs::OpenOptions;

// Builder pattern where methods are chained returning self

// let log = OpenOptions::new()
//     .append(true) // if file exists, add to the end
//     .open("server.log")?;

// let file = OpenOptions::new()
//     .write(true)
//     .create_new(true) // fail if file exists
//     .open("new_file.txt")?;

// Seeking

// pub trait Seek {
//     fn seek(&mut self, pos: SeekFrom) -> io::Result<u64>;
// }

// pub enum SeekFrom {
//     Start(u64),
//     End(i64),
//     Current(i64)
// }

// file.seek(SeekFrom::Start(0))
// file.seek(SeekFrom::Current(-8))

// Other Reader and Writer Types

// let stdin = io::stdin();
// let line = stdin.lock().lines(); // ok , io::stdin().lock() doesn't work

// io::stdout();
// io::stderr();
// Vec<u8>;
// Cursor::new(buf);
// std::net::TcpStream;
// std::process::Command;

// use std::process::{Command, Stdio};

// let mut child = 
//     Command::new("grep")
//     .arg("-e")
//     .arg("a.*e.*i.*o.*u")
//     .stdin(Stdio::piped())
//     .spawn()?;
// let mut to_child = child.stdin.take().unwrap();
// for work in my_words {
//     writeln!(to_child, "{}", word)?;
// }
// drop(to_child); // close grep's stdin, so it will exit
// child.wait()?;

// io::sink();
// io::empty();
// io::repeat(byte);

// Binary Data, Compression, and Serialization

// use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};

// let n = reader.read_u32::<LittleEndian>()?;
// writer.write_i64::<LittleEndian>(n as i64)?;

// use flate2::read::GzDecoder;
// let file = File::open("access.log.gz")?;
// let mut gzip_reader = GzDecoder::new(file);

// type RoomId = String;
// type RoomExits = Vec<(char, RoomId)>;
// type RoomMap = HashMap<RoomId, RoomExits>;

// // Create a simple map.
// let mut map = RoomMap::new();
// map.insert("Cobble Crawl".to_string(),
//             vec![('W', "Debris Room".to_string())]);
// map.insert("Debris Room".to_string(),
//             vec![('E', "Cobble Crawl".to_string()),
//                 ('W', "Sloping Canyon".to_string())]);

// serde_json::to_writer(&mut std::io::stdout(), &map)?;


// #[derive(Serialize, Deserialize)]
// struct Player {
//     location: String,
//     items: Vec<String>,
//     health: u32
// }

// [dependencies]
// serde={version="1.0", features=["derive"]}
// serde_json = "1.0"

// serde_json::to_writer(&mut std::io::stdout(), &player)?;