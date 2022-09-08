use std::io;


fn main() {
    println!("Which Fibonacci number you want to compute?");
    let n: u32 = read_unsigned_integer();
    let n_fib: u64 = fibonacci(n);
    println!("The {n}th Fibonacci number is {n_fib}.");
}


fn read_unsigned_integer() -> u32 {
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");

    input.trim().parse().expect("Not an unsigned integer!")
}


fn fibonacci(n: u32) -> u64 {
    if n == 0 || n == 1 {
        return 1
    }

    fibonacci(n - 1) + fibonacci(n - 2)
}
