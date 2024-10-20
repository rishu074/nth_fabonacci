use std::io;

fn gen_nth_number(n: u32) -> u32 {
    if n==0 {
        return 0
    } else if n==1 {
        return 1
    } else {
        return gen_nth_number(n-1) + gen_nth_number(n-2)
    }
}


fn main() {
    println!("Generate Fibonacci number. \n\nFirst numbers are assumed 0 and 1. \nPlease enter n");
    
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read the 'n'");

    let input: u32 = input.trim().parse().expect("Please enter a number");

    println!("Your Fibonacci number is {}", gen_nth_number(input));
}
