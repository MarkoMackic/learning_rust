use std::io;

fn fib(n: u32) -> u32 {
    if n == 1 || n == 0 {
        n
    }
    else
    { 
        fib(n-1) + fib(n-2)
    }
}

fn main() {
    println!("Idx of fib number ( must be positive)  : ");

    let mut fib_idx = String::new();

    io::stdin().read_line(&mut fib_idx).expect("Unable to read line!");

    let fib_idx: u32 = fib_idx.trim().parse().expect("Unable to convert to number");

    println!("Fib number at idx {} is {}", fib_idx, fib(fib_idx));
}
