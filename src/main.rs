use abjad::{Abjad, AbjadPrefs};
use std::io;

fn main() {
    // Introduce the program
    println!("This will calculate the abjad value of a string of Persian or Arabic text.");

    // The remainder will loop until quit
    loop {
        // Prompt user for input
        println!("Enter some text below; or enter the letter Q/q to quit.\n");

        // Read input to string
        let mut abjad_text = String::new();
        io::stdin()
            .read_line(&mut abjad_text)
            .expect("Failed to read input line");

        // Take a slice of the whole input string
        // I'm still not sure why it has to be this way
        // While we're at it, remove any leading or trailing whitespace
        let abjad_slice: &str = abjad_text[..].trim();

        // Abort early if user entered Q/q
        match abjad_slice {
            "Q" | "q" => break,
            _ => {}
        }

        // Get abjad value and list of unrecognized characters (if any)
        // Default abjad options are assumed in this CLI
        let (total, unrecognized) = abjad_slice.abjad_collect_errors(AbjadPrefs::default());

        // Print result
        println!("\nTotal abjad value:");
        println!("{}\n", total);

        // Report unrecognized characters (if any)
        if !unrecognized.is_empty() {
            let mut unrecognized_concat = String::new();

            for item in unrecognized.iter() {
                unrecognized_concat += item;
                unrecognized_concat += ", ";
            }

            println!("Double-check input; only Arabic and Persian letters will be counted.");
            println!("The following unrecognized characters were ignored:");
            println!("{}\n", unrecognized_concat.trim_end_matches(", "));
        }
    }
}
