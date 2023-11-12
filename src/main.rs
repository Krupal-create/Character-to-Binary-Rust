use std::io;

fn main() {

    println!("Enter a character:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    
    let input_char = input.trim().chars().next();

    match input_char {
        Some(c) => {

            let binary_representation = char_to_binary(c);
            

            println!("Binary representation of '{}' is: {}", c, binary_representation);
        }
        None => println!("No character entered."),
    }
}

fn char_to_binary(c: char) -> String {
    let mut result = String::new();

    for i in (0..8).rev() {
        let bit = (c as u8) & (1 << i) != 0;
        result.push(if bit { '1' } else { '0' });
    }

    result
}
