fn main() {
    let sum = add(5, 10);
    let difference = subtract(10, 5);

    println!("The sum is: {sum}");
    println!("The difference is: {difference}");
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn subtract(x: i32, y: i32) -> i32 {
    x - y
}