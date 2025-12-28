fn main() {
    println!("Hello, world!");
    let number = 3;

    if number < 5 {
        println!("condition was true");
        do_gg();
    } else {
        println!("condition was false");
    }
}

fn do_gg() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
}
