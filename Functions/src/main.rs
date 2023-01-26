fn main() {
    println!("{}",sum(1,2));
    println!("{}",take(7,5));
    println!("{}",multiply(7,5));
    println!("{}",divide(7,5));
}

fn sum(a:u32, b:u32) -> u32 {
    return a + b;
}

fn take(a:u32, b:u32) -> u32 {
    return a - b;
}

fn multiply(a:u32, b:u32) -> u32 {
    return a * b;
}

fn divide(a:u32, b:u32) -> u32 {
    return a / b;
}