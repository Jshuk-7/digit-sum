fn main() {
    loop {
        println!("Enter a number to get the digit sum: (type 'exit' to quit)");
        let mut num_as_text = String::new();
        std::io::stdin()
            .read_line(&mut num_as_text)
            .expect("Failed to read input!");
        num_as_text = num_as_text.trim().to_string();

        if num_as_text == "exit" {
            break;
        }

        match num_as_text.parse() {
            Ok(num) => println!("Digit Sum: {}", calcualte_digit_sum(num)),
            Err(_) => {
                eprintln!("Error: '{num_as_text}' is not a number!");
                continue;
            }
        }
    }
}

fn calcualte_digit_sum(num: i32) -> i32 {
    let num_as_string = num.to_string();

    let mut digits_to_add = vec![];

    for character in num_as_string.chars().into_iter() {
        let digit = character
            .to_digit(10)
            .expect("Failed to convert to a base decimal number!");

        digits_to_add.push(digit as u8)
    }

    let mut result = 0;

    for digit in digits_to_add.iter() {
        result += *digit as i32
    }

    result
}
