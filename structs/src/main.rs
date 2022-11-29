
struct GrayscaleMap {
    pixels: Vec<u8>,
    size: (usize, usize)
}

fn new_map(size: (usize, usize), pixels: Vec<u8>) -> GrayscaleMap {
    assert_eq!(pixels.len(), size.0 * size.1);
    GrayscaleMap { pixels, size }
}

fn main() {
    
    let width = 1024;
    let height = 576;
    let image = GrayscaleMap {
        pixels: vec![0; width * height],
        size: (width, height)
    };
    assert_eq!(image.size, (1024, 576));
    assert_eq!(image.pixels.len(), 1024 * 576);

    let hokey = Broom {
        name: "Hokey".to_string(),
        height: 60,
        health: 100,
        position: (100.0, 200.0, 0.0),
        intent: BroomIntent::FetchWater
    };

    let (hokey1, hokey2) = chop(hokey);
    assert_eq!(hokey1.name, "Hokey I");
    assert_eq!(hokey1.height, 30);
    assert_eq!(hokey1.health, 100);

    assert_eq!(hokey2.name, "Hokey II");
    assert_eq!(hokey2.height, 30);
    assert_eq!(hokey2.health, 100);

    let image_bounds = Bounds(1024, 768);
    assert_eq!(image_bounds.0 * image_bounds.1, 786432);

    let mut q = Queue { older: Vec::new(), younger: Vec::new() };

    q.push('0');
    q.push('1');
    assert_eq!(q.pop(), Some('0'));

    q.push('2');
    assert_eq!(q.pop(), Some('1'));
    assert_eq!(q.pop(), Some('2'));
    assert_eq!(q.pop(), None);

    assert!(q.is_empty());
    q.push('☉');
    assert!(!q.is_empty());

    let mut q = Queue { older: Vec::new(), younger: Vec::new() };

    q.push('P');
    q.push('D');
    assert_eq!(q.pop(), Some('P'));
    q.push('X');

    let (older, younger) = q.split();
    assert_eq!(older, vec!['D']);
    assert_eq!(younger, vec!['X']);

    let mut bq = Box::new(Queue::new());

    bq.push('D');

    let mut q = Queue::new();

    q.push('*');

    let mut q = Queue::new();
    let mut r = Queue::new();

    q.push("CAD");  // apparently a Queue<&'static str>
    r.push(0.74);   // apparently a Queue<f64>

    q.push("BTC");   // Bitcoins per USD, 2019-6
    r.push(13764.0); // Rust fails to detect irrational exuberance

    let a = [0, -3, 0, 15, 48];
    let e = find_extrema(&a);
    assert_eq!(*e.least, -3);
    assert_eq!(*e.greatest, 48);
}

struct Broom {
    name: String,
    height: u32,
    health: u32,
    position: (f32, f32, f32),
    intent: BroomIntent
}

#[derive(Copy, Clone)]
enum BroomIntent { FetchWater, DumpWater }

fn chop(b: Broom) -> (Broom, Broom) {
    let mut broom1 = Broom {height: b.height / 2, .. b };
    let mut broom2 = Broom { name: broom1.name.clone(), .. broom1 };

    broom1.name.push_str(" I");
    broom2.name.push_str(" II");

    (broom1, broom2)
}

struct Bounds(usize, usize);

pub struct Queue<T> {
    older: Vec<T>,
    younger: Vec<T>
}

impl<T> Queue<T> {
    // Type-Associated function
    pub fn new() -> Self {
        Self { older: Vec::new(), younger: Vec::new() }
    }


    // methods
    pub fn push(&mut self, t: T) {
        self.younger.push(t);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }

            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }

        self.older.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }

    pub fn split(self) -> (Vec<T>, Vec<T>) {
        (self.older, self.younger)
    }
}

impl Queue<f64> {
    fn sum(&self) -> f64 {
        34.50
    }
}

pub struct Vector2 {
    x: f32,
    y: f32,
}

impl Vector2 {
    const ZERO: Vector2 = Vector2 { x: 0.0, y: 0.0 };
    const UNIT: Vector2 = Vector2 { x: 1.0, y: 0.0 };

    const NAME: &'static str = "Vector2";
    const ID: u32 = 18;
}

struct Extrema<'elt> {
    greatest: &'elt i32,
    least: &'elt i32,
}

fn find_extrema<'s>(slice: &'s [i32]) -> Extrema<'s> {
    let mut greatest = &slice[0];
    let mut least = &slice[0];

    for i in 1..slice.len() {
        if slice[i] < *least { least = &slice[i]; }
        if slice[i] > *greatest { greatest = &slice[i]; }
    }
    Extrema { greatest, least }
}

struct Polynomial<const N: usize> {
    coefficients: [f64; N]
}

impl<const N: usize> Polynomial<N> {
    fn new(coefficients: [f64; N]) -> Polynomial<N> {
        Polynomial { coefficients }
    }

    fn eval(&self, x: f64) -> f64 {
        let mut sum = 0.0;
        for i in (0..N).rev() {
            sum = self.coefficients[i] + x * sum;
        }

        sum
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

pub struct SpiderRobot {
    species: String,
    web_enabled: bool,
    leg_devices: [fd::FileDesc; 8],
}

use std::rc::Rc;

pub struct SpiderSenses {
    robot: Rc<SpiderRobot>,
    eyes: [Camera; 32],
    motion: Accelerometer,
}