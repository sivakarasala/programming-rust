// Utility Traits

// Language extension traits
//      These serve as extension points, allowing to integrate
//      user defined types more closely with the language. Eg. Drop, From
// Marker traits
//      Mostly used to bound generic type variables to express
//      constraints which can't be captured easily. Eg. Sized & Copy
// Public vocabulary traits


// Drop -> Destructors. Cleanup code that rust runs automatically
//          whenever a value is dropped.

// Sized -> Marker trait for types with a fixed size known at
//          compile time, as opposed to types (such as slices)
//          that are dynamically sized.

// Clone -> Types that support cloning values.

// Copy -> Marker trait for types that can be cloned simply by making
//          a byte-for-byte copy of the memory containing the value.

// Deref and DerefMut -> Traits for smart pointer types.

// Default -> Types that have a sensible "default value."

// AsRef and AsMut -> Conversion traits for borrowing one type of reference
//                      from another.

// Borrow and BorrowMut -> Conversion traits, like AsRef/AsMut, but additionally
//                      guarenteeing consistent hashing, ordering, and equality.

// From and Into -> Conversion traits for transforming one type of value into another.

// TryFrom and TryInto -> Conversion traits for transforming one type of value
//                          into another, for transformations that might fail.

// ToOwned -> Conversion trait for converting a reference to an owned value.

// Iterator, IntoInterator, Hash, Send, Sync ...etc

struct Appellation {
    name: String,
    nicknames: Vec<String>
}

// trait Drop {
//     fn drop(&mut self);
// }

impl Drop for Appellation {
    fn drop(&mut self) {
        print!("Dropping {}", self.name);
        if !self.nicknames.is_empty() {
            print!(" (AKA {})", self.nicknames.join(", "));
        }
        println!("");
    }
}

// ?Sized often called as questionably sized
struct RcBox<T: ?Sized> {
    ref_count: usize,
    value: T
}
use std::fmt::Display;

fn display(boxed: &RcBox<dyn Display>) {
    println!("For your enjoyment: {}", &boxed.value);
}


// trait Clone: Sized {
//     fn clone(&self) -> Self;
//     fn clone_from(&mut self, source: &Self) {
//         *self = source.clone()
//     }
// }

// trait Copy: Clone {}

// impl Copy for MyType {}

// trait Deref {
//     type Target: ?Sized;
//     fn deref(&self) -> &Self::Target;
// }

// trait DerefMut: Deref {
//     fn deref_mut(&mut self) -> &mut Self::Target;
// }



use std::ops::{Deref, DerefMut};

struct Selector<T> {
    // elements available in this `Selector`
    elements: Vec<T>,

    // The index of the "current" element in `elements`. A `Selector`
    // behaves like a pointer to the current element.
    current: usize
}

impl<T> Deref for Selector<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.elements[self.current]
    }
}

impl<T> DerefMut for Selector<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.elements[self.current]
    }
}

trait Default {
    fn default() -> Self;
}

impl Default for String {
    fn default() -> String {
        String::new()
    }
}

trait AsRef<T: ?Sized> {
    fn as_ref(&self) -> &T;
}

trait AsMut<T: ?Sized> {
    fn as_mut(&mut self) -> &mut T;
}

trait Borrow<Borrowed: ?Sized> {
    fn borrow(&self) -> &Borrowed;
}

trait Into<T>: Sized {
    fn into(self) -> T;
}

trait From<T>: Sized {
    fn from(other: T) -> Self;
}

trait ToOwned {
    type Owned: Borrow<Self>;
    fn to_owned(&self) -> Self::Owned;
}

use std::collections::HashSet;

fn main() {
    let mut a = Appellation {
        name: "Zeus".to_string(),
        nicknames: vec!["cloud collector".to_string(),
                        "king of the gods".to_string()]
    };

    println!("before assignment");
    a = Appellation { name: "Hera".to_string(), nicknames: vec![] };
    println!("at end of block");

    let boxed_lunch: RcBox<String> = RcBox {
        ref_count: 1,
        value: "lunch".to_string(),
    };

    
    let boxed_displayable: &RcBox<dyn Display> = &boxed_lunch;
    display(&boxed_lunch);

    let mut s = Selector { elements: vec!['x', 'y', 'z'],
    current: 2 };

    // Because `Selector` implements `Deref`, we can use the `*` operator to
    // refer to its current element.
    assert_eq!(*s, 'z');

    // Assert that 'z' is alphabetic, using a method of `char` directly on a
    // `Selector`, via deref coercion.
    assert!(s.is_alphabetic());

    // Change the 'z' to a 'w', by assigning to the `Selector`'s referent.
    *s = 'w';

    assert_eq!(s.elements, ['x', 'y', 'w']);
    
    let squares = [4, 9, 16, 25, 36, 49, 64];
    let (powers_of_two, impure): (HashSet<i32>, HashSet<i32>)
        = squares.iter().partition(|&n| n & (n-1) == 0);

    assert_eq!(powers_of_two.len(), 3);
    assert_eq!(impure.len(), 4);

}
