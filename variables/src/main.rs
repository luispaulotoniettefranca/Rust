fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    let x = x + 1;
    println!("The value of x is: {}", x);
    let x = x * 2;
    println!("The value of x is: {}", x);

    let array: [i32; 6] = [1, 2, 3, 4, 5, 6];
    println!("The value of first array field is: {}", array[0]);
    println!("The value of second array field is: {}", array[1]);
    println!("The value of third array field is: {}", array[2]);
    println!("The value of fourth array field is: {}", array[3]);
    println!("The value of fifty array field is: {}", array[4]);
    println!("The value of sixty array field is: {}", array[5]);

    let tuple: (i32, char, f64, &str) = (32, 'a', 6.4, "Hello everybody!");
    println!("The value of first array field is: {}", tuple.0);
    println!("The value of second array field is: {}", tuple.1);
    println!("The value of third array field is: {}", tuple.2);
    println!("The value of fourth array field is: {}", tuple.3);

    let vector: Vec<i32> = vec![1, 2, 3, 4];
    println!("The value of first array field is: {}", vector[0]);
    println!("The value of second array field is: {}", vector[1]);
    println!("The value of third array field is: {}", vector[2]);
    println!("The value of fourth array field is: {}", vector[3]);
}
