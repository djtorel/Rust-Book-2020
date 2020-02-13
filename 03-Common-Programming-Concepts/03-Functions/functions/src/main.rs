fn main() {
    println!("Hello, world!");

    another_function(5, 6);

    println!("The value is {}", five());

    println!("The value of x plus one is {}", plus_one(5));
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
