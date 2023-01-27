fn main() {
    println!("{}", you_here(1,5));
}

fn you_here(power:u32, speed:u32) -> bool {
    if power == 0  || speed == 0 {
        return false;
    } else if power > 20 || speed > 20 {
        return true;
    } else {
        return false;
    }
}