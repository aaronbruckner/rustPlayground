// Runs through a variety of basic rust features
fn main() {
    println!("Fib 3: {} Fib 10: {}", fib(3), fib(10));
    print_loop(3);
    strings(5);
    area();
}


fn fib(n: i32) -> i32 {
    if n <= 0 {
        0
    } else if n == 1 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fn print_loop(n: i32) {
    for i in 0..n {
        println!("Loop: {}", i);
    }
}

fn strings(n: usize) {
    let str: String = String::from("hello world!");

    let str_slice = &str[0..n];
    println!("String slice: {}", str_slice);
}

fn area() {
    println!("Area with params: {}", area_params(3, 4));
    println!("Area with tuple: {}", area_tuple((3,4)));
    let rec = Rectangle {
        height: 3,
        width: 4
    };
    println!("Area with struct: {}", area_struct(&rec));
    println!("Area with struct method: {}", rec.area());

    println!("Use rec after borrow: {}", rec.height);
    println!("Rec Debug: {:#?}", rec);
}

fn area_params(height: i32, width: i32) -> i32 {
    height * width
}

fn area_tuple(dimensions: (i32, i32)) -> i32 {
    dimensions.0 * dimensions.1
}
#[derive(Debug)]
struct Rectangle {
    height: i32,
    width: i32
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.height * self.width
    }
}

fn area_struct(rec: &Rectangle) -> i32 {
    rec.height * rec.width
}