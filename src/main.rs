use std::io;

fn main() {
    // Initial description of the program
    println!("This utility will calculate the total abjad value of a string of text in Arabic or Persian.");

    // The remainder will loop until quit
    loop {
        // Prompt the user for input
        println!("Enter some text below; or enter the letter Q/q to quit.");

        // Read the input and turn it into a String variable
        let mut abjad_text = String::new();
        io::stdin()
            .read_line(&mut abjad_text)
            .expect("Failed to read line…");

        // Slice the whole input String
        // I'm still not sure why it had to be done this way
        let abjad_text_slice: &str = &abjad_text[..].trim();

        // Abort early if the user entered Q/q
        match abjad_text_slice {
            "Q" | "q" => break,
            _ => {}
        }

        // Turn our sliced string into a vector of characters
        let abjad_text_vector: Vec<char> = abjad_text_slice.chars().collect();

        // Create a variable to hold the total abjad value
        let mut abjad_total = 0;

        // Create a Boolean variable to toggle the error message for unrecognized input
        let mut unrecognized_input = false;

        // Iterate through the vector one character at a time
        for c in abjad_text_vector {
            // Take the Unicode escape sequence of each character and turn it into a string
            let abjad_char: &str = &c.escape_unicode().to_string();

            // Match each letter to its abjad value, and add that value to the total
            match abjad_char {
                r"\u{621}" | r"\u{622}" | r"\u{623}" | r"\u{625}" | r"\u{627}" | r"\u{671}" => {
                    abjad_total += 1
                }
                r"\u{628}" | r"\u{67e}" => abjad_total += 2,
                r"\u{62c}" | r"\u{686}" => abjad_total += 3,
                r"\u{62f}" => abjad_total += 4,
                r"\u{629}" | r"\u{647}" | r"\u{6c0}" => abjad_total += 5,
                r"\u{624}" | r"\u{648}" => abjad_total += 6,
                r"\u{632}" | r"\u{698}" => abjad_total += 7,
                r"\u{62d}" => abjad_total += 8,
                r"\u{637}" => abjad_total += 9,
                r"\u{626}" | r"\u{649}" | r"\u{64a}" | r"\u{6cc}" => abjad_total += 10,
                r"\u{643}" | r"\u{6a9}" | r"\u{6af}" => abjad_total += 20,
                r"\u{644}" => abjad_total += 30,
                r"\u{645}" => abjad_total += 40,
                r"\u{646}" => abjad_total += 50,
                r"\u{633}" => abjad_total += 60,
                r"\u{639}" => abjad_total += 70,
                r"\u{641}" => abjad_total += 80,
                r"\u{635}" => abjad_total += 90,
                r"\u{642}" => abjad_total += 100,
                r"\u{631}" => abjad_total += 200,
                r"\u{634}" => abjad_total += 300,
                r"\u{62a}" => abjad_total += 400,
                r"\u{62b}" => abjad_total += 500,
                r"\u{62e}" => abjad_total += 600,
                r"\u{630}" => abjad_total += 700,
                r"\u{636}" => abjad_total += 800,
                r"\u{638}" => abjad_total += 900,
                r"\u{63a}" => abjad_total += 1000,
                // Ignore spaces and ZWNJ
                r"\u{20}" | r"\u{200c}" => {}
                // Handle remaining cases
                _ => {
                    // Let the user know there was unrecognized input
                    println!("The Unicode character {} has been ignored.", abjad_char);
                    // Set the Boolean for the error message
                    unrecognized_input = true;
                }
            }
        }

        // Print the result
        println!("Total abjad value:");
        println!("{}", abjad_total);
        // Print the error message
        if unrecognized_input {
            println!("Please double-check your input. Only normal Arabic and Persian letters can be processed.");
        }
    }
}
