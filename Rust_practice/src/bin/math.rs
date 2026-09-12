fn main() {
    let sum = add(5, 10);
    let difference = subtract(10, 5);
    let product = multiply(4, 10);
    let division = quotient(10, 2);

    println!("The sum is: {sum}");
    println!("The difference is: {difference}");
    println!("The product is: {product}");
    println!("The quotient is: {division}");
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn subtract(x: i32, y: i32) -> i32 {
    x - y
}

fn multiply(x: i32, y: i32) -> i32 {
    x * y
}


fn variable() {
    let x = 7;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}