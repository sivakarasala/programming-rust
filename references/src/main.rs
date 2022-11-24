use std::collections::HashMap;

// A reference lets you access a value without affecting its
// ownership. Two types of references:
// 1. Shared Reference lets you read but not modify its referent
// 2. Mutable reference lets you both read and modify the value.
// Kind of exclusive to each other.

// Lifetimes
// A lifetime is some stretch of your program for which a reference
// could be safe to use: a statement, an expression, the scope of 
// some variable, or the like.

// A variable's lifetime must contain or enclose that of the reference
// borrowed from it.

// If a reference is stored in a variable, then the reference's type 
// must be good for the entire lifetime of the variable, from its
// initialization until its last use.

type Table = HashMap<String, Vec<String>>;

fn show(table: &Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!("  {}", work);
        }
    }
}

fn sort_works(table: &mut Table) {
    for (_artist, works) in table {
        works.sort();
    }
}

fn main() {
    let mut table = Table::new();
    table.insert("Gesualdo".to_string(),
                vec!["may madrigals".to_string(),
                    "Tenebrae Responsoria".to_string()]);
    table.insert("Caravaggio".to_string(),
                vec!["The Musicians".to_string(),
                    "The Calling of St. Matthew".to_string()]);
    table.insert("Cellini".to_string(),
                vec!["Perseus with the head of Medusa".to_string(),
                    "a salt cellar".to_string()]);
    show(&table);
    
    
    let x = 10;
    let r = &x;
    assert!(*r == 10);

    let mut y = 32;
    let m = &mut y;
    *m += 32;
    assert!(*m == 64);

    struct Anime { name: &'static str, bechdel_pass: bool }
    let aria = Anime { name: "Aria: The Animation", bechdel_pass: true };
    let anime_ref = &aria;
    assert_eq!(anime_ref.name, "Aria: The Animation");

    // Equivalent to the above, but with the dereference written out:
    assert_eq!((*anime_ref).name, "Aria: The Animation");

    let mut v = vec![1973, 1968];
    v.sort();
    (&mut v).sort();

    let x = 10;
    let y = 20;
    let mut r = &x;
    let b = true;

    if b { r = &y; }

    assert!(*r == 10 || *r == 20);

    struct Point { x: i32, y: i32 }
    let point = Point { x: 1000, y: 729 };
    let r: &Point = &point;
    let rr: &&Point = &r;
    let rrr: &&&Point = &&r;

    assert_eq!(rrr.y, 729);


    let x = 10;
    let y = 10;

    let rx = &x;
    let ry = &y;

    let rrx = &rx;
    let rry = &ry;

    assert!(rrx <= rry);
    assert!(rrx == rry);

    assert!(!std::ptr::eq(rx, ry));
    
    // assert!(rx == rrx);
    assert!(rx == *rrx);

    {
        struct S<'a, 'b> {
            x: &'a i32,
            y: &'b i32
        }

        let x = 10;
        let r;
        {
            let y = 20;
            {
                let s = S { x: &x, y: &y };
                r = s.x;
            }
        }
        println!("{}", r);
    }

    {
        let mut wave = Vec::new();
        let head = vec![0.0, 1.0];
        let tail = [0.0, -1.0];

        extend(&mut wave, &head);
        extend(&mut wave, &tail);

        assert_eq!(wave, vec![0.0, 1.0, 0.0, -1.0]);

        // Shared access is read-only access.
        // Mutable access is exclusive access.
        //extend(&mut wave, &wave);

        // A data race is possible only when some value is both
        // mutable and shared between threads -- which is exactly
        // what Rust's reference rules eliminate.

        // A concurrent rust program that avoids unsafe code is
        // free of data races by construction.
        
    }
}

static mut STASH: &i32 = &128;
fn f(p: &'static i32) { 
    unsafe {
        STASH = p;
    } 
}

fn extend(vec: &mut Vec<f64>, slice: &[f64]) {
    for elt in slice {
        vec.push(*elt);
    }
}