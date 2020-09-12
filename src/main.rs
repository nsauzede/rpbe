use std::ops::{Add, Mul};

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

impl Add<Point> for Point {
    type Output = Point;
    fn add(self, p: Point) -> Point {
        Point {
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }
}

impl Mul<Point> for Point {
    type Output = i32;
    fn mul(self, p: Point) -> i32 {
        self.x * p.y - self.y * p.x
    }
}

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
        } => println!(
            "Divisor 
         is zero"
        ),
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

impl BitSet for u64 {
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

struct Type {
    v: u64,
}

impl BitSet for Type {
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

impl Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.write_fmt(format_args!("->{}<-", self.v))
    }
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

    let mut n = 42;
    println!("n={}", n);
    let n2 = !n;
    println!("!n={}", n2);

    let mut n = Type { v: 42 };
    println!("n={:?}", n);
    n.clear(3);
    println!("n={:?}", n);
    n.toggle(3);
    println!("n={:?}", n);
    n.toggle(3);
    println!("n={:?}", n);

    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = p1 + p2;
    let dot = p1 * p2;
    println!("p3={:?}", p3);
    println!("p1={:?}", p1);
    println!("p2={:?}", p2);
    println!("dot={}", dot);
}
