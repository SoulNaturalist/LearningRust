
const h: [i32; 7] = [1, 2, 3, 5, 8, 13, 21];


fn main () {
    // test2()
    test3()

}

fn test2 () {
    let mut t = 0;
    while t < 10 {
        println!("{}", t);
        t += 1;
    }
}

fn test () -> i32 {
    loop {
        let test = test();
        println!("{}", test);
        if test == 1 {
            break;
        }
    }
    let test = 1;
    test + 1

}

fn test3 () {
    let mut vec2: Vec<i32> = Vec::new();
    for el in h {
        println!("{}", el);
        vec2.push(el * 2);
    }
    println!("{:?}", vec2);

}
