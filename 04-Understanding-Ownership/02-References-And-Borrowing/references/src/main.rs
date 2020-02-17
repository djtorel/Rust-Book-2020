fn main() {
    let mut s1 = String::from("hello");

    change(&mut s1);

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    change(&mut s1);
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a string
    s.len()
} // Here, s goes out of scope. But it does not have ownership of what
  // it refers to, nothing happens.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
