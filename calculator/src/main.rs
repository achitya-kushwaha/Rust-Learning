use std::io;
fn main() {
    println!("Welcome to Banaras Tech's Rust calculator!");
    // Getting number 1
    println!("Enter number 1:");
    let mut num1: String = String::new();
    io::stdin().read_line(&mut num1).expect("Failed to read line. Try again ..");
    let n1: i32 = num1.trim().parse().unwrap();
    // Getting number 2
    println!("Enter number 2:");
    let mut num2: String = String::new();
    io::stdin().read_line(&mut num2).expect("Failed to read line. Try again ..");
    let n2: i32 = num2.trim().parse().unwrap();
    // Getting what to do
    println!("Enter number expression:");
    let mut exp: String = String::new();
    io::stdin().read_line(&mut exp).expect("Failed to read the expression. Try Againg ..");
    let ex = exp.trim();
    // Checking the expression
    let ans: i32 = if ex == "+" {
        add(n1, n2)
    } else if ex == "-" {
        sub(n1, n2)
    } else if ex == "*" {
        mul(n1, n2)
    } else if ex == "/" {
        div(n1, n2)
    } else {
        1
    };
    println!("Answer of {} {} {} = {}", n1 , ex , n2 , ans);
}
fn add (n1: i32 , n2: i32) -> i32 {
    let answer: i32 = n1 + n2;
    return answer;
}
fn sub (n1: i32 , n2: i32) -> i32 {
    let answer: i32 = n1 - n2;
    return answer;
}
fn mul (n1: i32 , n2: i32) -> i32 {
    let answer: i32 = n1 * n2;
    return answer;
}
fn div (n1: i32 , n2: i32) -> i32 {
    let answer: i32 = n1 / n2;
    return answer;
}