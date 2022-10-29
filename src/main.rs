use std::io;

fn main() {
    println!("Tell me which number of Fibonacci do you want:");
    println!("To end the program type `exit`");
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(& mut input)
            .expect("Could not read input");
        if input.trim() == "exit" {
            break;
        }
        let input: u32 = match input.trim()
            .parse() {
                Ok(input) => input,
                Err(_) => continue
            };
        println!("Fibonacci ({}) => {}", input, fib(input - 1));
    }
}

fn fib(n: u32) -> u32{
    if n <= 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } fib(n - 1) + fib(n - 2)
}
