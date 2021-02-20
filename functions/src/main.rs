fn main() {
    print_formatted();
}

fn five() -> u32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn print_message() {
    print!("Hello guys!");
}

fn print_formatted() {
    print_message();
    println!(" {} + {} = {}", five(), 1, plus_one(5));
}