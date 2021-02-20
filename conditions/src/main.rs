fn main() {
    let x: bool = false;
    let y: bool = true;
    let z: bool = if x {x} else {y};
    if z {
        println!("x variable is true");
    } else {
        println!("x variable is false");
    }

    let [a, b, c] = [16, 15, 49];
    if a % 4 != 0 {
        println!("{} isn't divisible by 4", a);
    } else if b % 5 != 0 {
        println!("{} isn't divisible by 5", b);
    } else if c % 7 != 0 {
        println!("{} isn't divisible by 7", c);
    } else {
        println!("{} is divisible by 4", a);
        println!("{} is divisible by 4", b);
        println!("{} is divisible by 4", c);
    }
}
