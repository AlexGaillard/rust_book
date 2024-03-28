use std::io;

fn main() {
    let input: u32;
    let mut loop_number = 0;
    let mut n1: u32 = 0;
    let mut n2: u32 = 1;

    loop {
        println!("Enter nth fibonacci number");

        let mut input_str = String::new();

        io::stdin()
            .read_line(&mut input_str)
            .expect("Failed to read input");

        match input_str.trim().parse() {
            Ok(num) => {
                if num > 2 {
                    input = num - 1;
                } else {
                    input = num;
                }
                break;
            }
            Err(_) => {
                println!("Please input a number!");
                continue;
            }
        };
    }

    loop {
        loop_number = loop_number + 1;
        let current: u32 = n1 + n2;

        if loop_number == input {
            println!("Your fibonacci number is: {}", current);
            break;
        }

        n1 = n2;
        n2 = current;
    }
}
