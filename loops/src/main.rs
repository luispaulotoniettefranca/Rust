fn main() {
    let mut x = 0;
    loop {
        if x == 5 {
            println!("Loop is ended");
            break;
        }
        println!("{}", x);
        x += 1;
    }

    let mut y = 0;
    while y != 5 {
        println!("{}", y);
        y += 1;
    }
    println!("While is ended");

    for z in 0..5 {
        println!("{}", z);
    }
    println!("For is ended");

    let a = [1, 2, 3, 4];
    for x in a.iter() {
        println!("{}", x);
    }
}
