use std::ops::{Add, Div, Mul, Sub};

type Scalar = f32;
//type Scalar = f64;
type V3 = [Scalar; 3];

macro_rules! v3 {
    ($x:expr, $y:expr, $z:expr) => {
        [$x as Scalar, $y as Scalar, $z as Scalar]
    };
}

fn first<T>(slice: &[T]) -> &T {
    &slice[0]
}

fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

macro_rules! point {
    ($x:expr, $y: expr) => {
        Point { x: $x, y: $y }
    };
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
    fn dist_from_origin(&self) -> f64 {
        let sum_of_squares = self.x.pow(2) + self.y.pow(2);
        (sum_of_squares as f64).sqrt()
    }
    fn translate(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }
}

macro_rules! impl_op {
    ($uo:ident, $lo:ident, $s:ident : $l:ty, $o:ident : $r:ty, $ot:ty $b:block) => {
        impl $uo<$r> for $l { type Output = $ot; fn $lo($s, $o: $r) -> $ot {$b} }
    };
}

impl_op!(Add, add, self: Point, o: Point, Point { Point { x: self.x + o.x, y: self.y + o.y }});
impl_op!(Add, add, self: Point, o: i32, Point { Point { x: self.x + o, y: self.y + o }});
impl_op!(Add, add, self: i32, o: Point, Point { Point { x: self + o.x, y: self + o.y }});
impl_op!(Sub, sub, self: Point, o: Point, Point { Point { x: self.x + o.x, y: self.y + o.y }});
impl_op!(Mul, mul, self: Point, o: i32, Point { Point { x: self.x * o, y: self.y * o }});
impl_op!(Mul, mul, self: i32, o: Point, Point { Point { x: self * o.x, y: self * o.y }});
impl_op!(Mul, mul, self: Point, o: Point, i32 { self.x * o.y - self.y * o.x });
impl_op!(Div, div, self: Point, o: i32, Point { Point { x: self.x / o, y: self.y / o }});

#[derive(Debug, Clone, Copy)]
struct Line {
    a: Point,
    b: Point,
}

fn print_line(line: Line) {
    println!("print_line : {:#?}", line);
}

fn inc_x(p: &mut Point) {
    p.x += 1;
}

fn mysplit_at(s: &str, i: usize) -> (&str, &str) {
    (&s[0..i], &s[i..])
}

#[derive(Debug)]
enum Expr {
    Null,
    Add(i32, i32),
    Sub(i32, i32),
    Mul(i32, i32),
    Div { dividend: i32, divisor: i32 },
    Val(i32),
}

fn print_expr(expr: &Expr) {
    match expr {
        Expr::Null => println!("No value"),
        Expr::Add(x, y) => println!("Add {}", x + y),
        Expr::Sub(x, y) => println!("Sub {}", x - y),
        Expr::Mul(x, y) => println!("Mul {}", x * y),
        Expr::Div {
            dividend: x,
            divisor: 0,
        } => println!("Divisor is zero"),
        Expr::Div {
            dividend: x,
            divisor: y,
        } => println!("Div {}", x / y),
        Expr::Val(x) => println!("Val {}", x),
    }
}

trait BitSet {
    fn clear(&mut self, index: usize);
    fn is_set(&self, index: usize) -> bool;
    fn set(&mut self, index: usize);
    fn toggle(&mut self, index: usize) {
        if self.is_set(index) {
            self.clear(index);
        } else {
            self.set(index);
        }
    }
}

macro_rules! int_bitset {
    ($ty:ty) => {
        impl BitSet for $ty {
            fn clear(&mut self, index: usize) {
                *self &= !(1 << index);
            }
            fn is_set(&self, index: usize) -> bool {
                (*self >> index) & 1 == 1
            }
            fn set(&mut self, index: usize) {
                *self |= 1 << index;
            }
            fn toggle(&mut self, index: usize) {
                *self ^= 1 << index;
            }
        }
    };
}

int_bitset!(u32);
int_bitset!(i32);

struct MType {
    v: u64,
}
macro_rules! mtype {
    ($v:expr) => {
        MType { v: $v }
    };
}
impl BitSet for MType {
    fn clear(&mut self, index: usize) {
        (*self).v &= !(1 << index);
    }
    fn is_set(&self, index: usize) -> bool {
        ((*self).v >> index) & 1 == 1
    }
    fn set(&mut self, index: usize) {
        (*self).v |= 1 << index;
    }
}

use std::fmt::Debug;

impl Debug for MType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.write_fmt(format_args!("->{}<-", self.v))
    }
}

fn index<T: PartialEq>(slice: &[T], target: &T) -> Option<usize> {
    for (index, element) in slice.iter().enumerate() {
        if element == target {
            return Some(index);
        }
    }
    None
}

fn min_max<T: PartialOrd + Copy>(slice: &[T]) -> Option<(T, T)> {
    if slice.is_empty() {
        return None;
    }
    let mut min = slice[0];
    let mut max = slice[0];
    for &element in slice {
        if element > max {
            max = element;
        }
        if element < min {
            min = element;
        }
    }
    Some((min, max))
}

fn main() {
    let a = [1, 2, 3, 4];
    for e in &a {
        println!("e={}", e);
    }
    let n: usize = 2;
    println!("a2={}", a[n]);
    println!("Hello, world!");
    let mut a = 40;
    let mut b = 15;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    println!("Greatest common divisor of 15 and 40 is: {}", a);
    println!("max is {}", max(40, 15));
    println!("max is {}", max(40.001, 40.0015));
    println!("max is {}", max('b', 'a'));

    //    let point = Point { x: 24, y: 42 };
    //    println!("{:?}", point);

    let mut line = Line {
        //        a: Point { x: 24, y: 42 },
        a: Point::new(24, 42),
        b: Point { x: 12, y: 34 },
    };
    inc_x(&mut line.a);
    line.a.translate(3, 6);
    print_line(line);
    println!("line={:#?}", line);
    println!("dist={}", line.a.dist_from_origin());
    let (hello, world) = mysplit_at("helloworld", 5);
    println!("{}, {}!", hello, world);

    let quotient = Expr::Div {
        dividend: 10,
        divisor: 2,
    };
    let sum = Expr::Add(40, 2);
    print_expr(&quotient);
    print_expr(&sum);
    println!("quotient={:#?}", quotient);
    println!("sum={:#?}", sum);
    for i in 0..4 {
        println!("i={}", i);
    }
    println!("{}", 'A');
    println!("{}", b'A');

    let c = b'x';
    let up = match c {
        b'a'..=b'z' => c - 32,
        _ => c,
    };
    println!("c={} up={}", c, up);
    let up = if let b'a'..=b'z' = c { c - 32 } else { c };
    println!("c={} up={}", c, up);

    let n = 42;
    println!("n={}", n);
    let n2 = !n;
    println!("!n={}", n2);

    let mut n = mtype!(42);
    println!("n={:?}", n);
    n.clear(3);
    println!("n={:?}", n);
    n.toggle(3);
    println!("n={:?}", n);
    n.toggle(3);
    println!("n={:?}", n);

    let mut n = 42 as u32;
    println!("n={:?}", n);
    n.clear(3);
    println!("n={:?}", n);
    n.toggle(3);
    println!("n={:?}", n);
    n.toggle(3);
    println!("n={:?}", n);

    let p1 = point!(1, 2);
    let p2 = point!(3, 4);
    let p3 = p1 + p2;
    let dot = p1 * p2;
    println!("p3={:?}", p3);
    println!("p1={:?}", p1);
    println!("p2={:?}", p2);
    println!("dot={}", dot);
    println!("sub={:?}", p1 - p2);
    println!("muls={:?}", p1 * 666);
    println!("adds={:?}", p1 + 666);
    println!("adds={:?}", 333 + p1);
    println!("divs={:?}", (666 * p1) / 42);

    let v: V3 = v3!(1. + 0.1, 2.2, 3.3);
    println!("v3={:?}", v);
    println!("first={}", first(&v[1..]));
    let v2 = vec![1.1, 2.2, 3.3];
    println!("first={}", first(&v2));

    for i in &[42; 2] {
        println!("answer={}", i);
    }

    for (i, v) in vec![1, 2].iter().enumerate() {
        println!("answer{}={}", i, v);
    }
    macro_rules! print_val_index {
        ($val:expr,$index:expr) => {
            println!(
                "index of {} is {}",
                $val,
                match $index {
                    Some(i) => i.to_string(),
                    _ => "not found".to_string(),
                }
            )
        };
    }
    {
        let val = 2.2;
        let index = index(&v2, &val);
        print_val_index!(val, index);
    }
    let val = 3.1415;
    let index = index(&v2, &val);
    print_val_index!(val, index);
    macro_rules! print_minmax {
        ($v:expr) => {
            println!(
                "minmax={}",
                match min_max(&$v[..]) {
                    Some((min, max)) => format!("({};{})", min, max),
                    _ => "N/A".to_string(),
                }
            );
        };
    }
    print_minmax!(v);
    print_minmax!([0; 0]);

    macro_rules! hash {
        ($($key:expr => $value:expr),* $(,)* ) => {
    {
        let mut h = ::std::collections::HashMap::new();
        $(h.insert($key, $value);)*
        h
    }
        };
    }

    let mut h = ::std::collections::HashMap::new();
    h.insert("hello", "world");
    println!("hashmap={:?}", h);
    let h = hash!(
    "bonjour" => "monde",
    "bye" => "world",
    );
    println!("hashmap={:?}", h);

    #[derive(Debug)]
    struct Tetro {
        n: String,
    }
    #[derive(Debug)]
    struct A {
        curr: Option<Tetro>,
        next: Option<Tetro>,
    }
    let mut a = A {
        curr: None,
        next: None,
    };
    a.next = Some(Tetro {
        n: "42".to_string(),
    });
    if a.curr.is_none() {
        if let Some(tetro) = a.next.take() {
            a.curr = Some(tetro);
            a.next = Some(Tetro {
                n: "43".to_string(),
            });
        }
    }
    println!("a={:?}", a);
}
