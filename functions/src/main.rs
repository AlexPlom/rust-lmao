fn main() {
    println!("Hello, world!");

    another_function(69);
    print_label_measurement(42, 'm');
    let five = five();

    println!("The value of five is: {five}");
}

fn another_function(x: i32) {
    println!("Another function. {x}");
}

fn print_label_measurement(amount: i32, unit: char) {
    println!("The measurement is {amount}{unit}");
}

fn five() -> i32 {
    5
}
