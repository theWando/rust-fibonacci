use std::io;

fn main() {
    loop {
        println!("nth fibonacci: ");
        let mut nth = String::new();
        io::stdin().read_line(&mut nth)
            .expect("Error reading line");

        let nth: i32 = match nth.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let result = fibonacci(nth, 0 , 1);
        println!("\nthe fibonacci sequence is {}", result);
        break;
    }
}

fn fibonacci(value: i32, a: i32, b: i32) -> i32 {
    if value == 0 {
        return a;
    } else if value == 1 {
        return b;
    }
    fibonacci(value -1, b, a + b)
}
