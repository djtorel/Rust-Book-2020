fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction;
    let difference = 95.5 - 4.3;

    // multiplication
    let production = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    // boolean
    let t = true;
    let f: bool = false;

    // char
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("the value of y is: {}", y);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // array
    let a = [1, 2, 3, 4, 5];

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5];
    let a: [i32; 5];

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
