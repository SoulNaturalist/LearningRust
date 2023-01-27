fn main() {
    println!("{}", calc("+",5,15));
}

fn calc(operator:&str, a:u32, b:u32) -> u32 {
    if operator == "-" {
        match a.checked_sub(b) {
            Some(result) => result,
            None => std::u32::MAX,
        }
    } else if operator == "+" {
        a + b
    } else if operator == "/" {
        if b == 0 {
            return std::u32::MAX;
        }
        return a / b;
    } else {
        return a * b;
    }
}