fn main() {
    println!("Hello, world!");

    println!("The function is {}", five());
}

fn another_function(x: i32, unit_label: char) {
    println!("The measurement is: {}{}", x, unit_label);
}

fn five() -> i32 {
    5
}