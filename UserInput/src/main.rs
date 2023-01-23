use std::io;


fn main() {
    let mut name = String::new();
    let mut age = String::new();
    let mut city = String::new();
    println!("Введите свое имя: ");
    io::stdin().read_line(&mut name);
    println!("Введите ваш возраст: ");
    io::stdin().read_line(&mut age);
    println!("Введите ваш город: ");
    io::stdin().read_line(&mut city);
    println!("Имя: {} возраст: {} город: {}", name,age,city);
}