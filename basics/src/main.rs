// Runs through a variety of basic rust features
fn main() {
    println!("Fib 3: {} Fib 10: {}", fib(3), fib(10));
    print_loop(3);
    strings(5);
    area();
    enum_test();
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
    let rec1 = Rectangle {
        height: 3,
        width: 4
    };
    println!("Area with struct: {}", area_struct(&rec1));
    println!("Area with struct method: {}", rec1.area());

    println!("Use rec after borrow: {}", rec1.height);
    println!("Rec Debug: {:#?}", rec1);

    let rec2 = Rectangle {
        width: 1,
        height: 1
    };

    let rec3 = Rectangle {
        width: 10,
        height: 1
    };

    println!("Rec1 can hold rec2: {}", rec1.can_hold(&rec2));
    println!("Rec1 can hold rec3: {}", rec1.can_hold(&rec3));

    let square = Rectangle::square(3);
    println!("Square: {:#?}", square);
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
    fn square(n: i32) -> Rectangle {
        Rectangle {
            width: n,
            height: n
        }
    }
    fn area(&self) -> i32 {
        self.height * self.width
    }

    fn can_hold(&self, rec: &Rectangle) -> bool {
        self.height >= rec.height && self.width >= rec.width
    }
}

fn area_struct(rec: &Rectangle) -> i32 {
    rec.height * rec.width
}

#[derive(Debug)]
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write (String)
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(String)
}

fn enum_test() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Write(String::from("test")));
    println!("{:?}", Message::Move{x: 1, y: 2});

    let coin = Coin::Quarter(String::from("Washington"));
    println!("Penny is worth: {}", find_coin_value(&Coin::Penny));
    println!("Quarter is worth: {}", find_coin_value(&coin));
    println!("{}", find_coin_value(&coin));
}

fn find_coin_value(coin: &Coin) -> i32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quarter found for state: {}", state);
            25
        }
    }
}