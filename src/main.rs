// This program will convert a 32-bit integer into human readable words.
// This program uses loops to generate a 'prefix' (one - nine hundred nintey nine) and a 'suffix' (thousand, million, billion, etc...)
// Given a number like 1355823:
// one million, three hundred fifty five thousand, eight hundred twenty three
// (one) [million], (three hundred fifty five) [thousand], (eight hundred twenty three)
// (prefix) [suffix], (prefix) [suffix], (prefix)

use std::io;

// These constants are the 'dictionaries' we will use to turn the number into words
// ONES[1] == "one", TENS[2] == "twenty", etc...
// Note ONES goes all the way to 19, since english uses an irregular word formatting for 10-19.
const ONES: &[&str] = &["","one","two","three","four","five","six","seven","eight","nine","ten","eleven","twelve","thirteen","fourteen","fifteen","sixteen","seventeen","eighteen","nineteen"];
// TENS[1] should not be used since ONES[10] already has that information
const TENS:  &[&str] = &["","ten","twenty","thirty","fourty","fifty","sixty","seventy","eighty","ninety"];
const SUFFIX:  &[&str] = &["", "thousand", "million", "billion"];

// This function uses recursion to generate the 'prefix' used in the final string
// This function should be able to produce any string between "one" and "nine hundred ninty nine"
// This function expects an input 'x' less than 1000, and will error if this is not true
fn generate_number_prefix(x: i32, s: &mut String) {
    // Our main function already handles "zero" as the input; this check is to avoid extra spaces in the string
    if x == 0 {
        return;
    } else if x < 20 {
        s.push_str(ONES[x as usize]);
        s.push_str(" ");
    } else if x < 100 {
        s.push_str(TENS[(x/10) as usize]);
        s.push_str(" ");
        generate_number_prefix(x % 10, s);
    // We reuse the ONES array and simply append "hundred", thus generating strings like "three hundred"
    } else if x < 1000 {
        s.push_str(ONES[(x/100) as usize]);
        s.push_str(" hundred ");
        generate_number_prefix(x % 100, s);
    // If the input to this function is 1000 or greater, then we have a problem...
    } else {
        panic!("Something went wrong generating a prefix. Input must be less than 1000.")
    }
}

// Our main function which accepts a user input, and outputs the final string
fn main() {
    // Keep looping until the user quits with ctrl + c
    loop {
        println!("Please input a 32-bit integer:");

        // User input is a string
        let mut user_input = String::new();

        io::stdin().read_line(&mut user_input)
            .expect("Failed to read line.");

        // Convert string to 32-bit integer, otherwise return an error and restart the loop
        let mut user_input: i32 = match user_input.trim().replace(",","").parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Couldn't parse that value... Try again.");
                continue;
            },
        };

        // Handle the "zero" input
        if user_input == 0 {
            println!("zero");
            continue;
        // Handle -2^31 which cannot be converted to a positive number using i32
        } else if user_input == -2147483648 {
            println!("negative two billion, one hundred fourty seven million, four hundred eighty three thousand, six hundred fourty eight");
            continue;
        }

        // Check if the user inputted number is negative, store that information, and convert to positive
        let is_negative = {
            if user_input < 0 {
                // Note that this can cause an overflow panic when converting -2^31 to a positive number. We handle this above.
                user_input *= -1;
                true
            } else {
                false
            }
        };
        
        // This is the final output string
        let mut output = String::new();
        // This tracks what suffix we should use 
        let mut suffix_tracker = 0;

        // We break down the user's input by 'prefix' sections (i.e. dividing by 1000)
        // We take the remainder of 'user_input/1000', and process that number through 'generate_number_prefix()'
        // We then set 'user_input = user_input / 1000', and repeat this until 'user_input == 0'
        while user_input > 0 {
            // This is a temporary variable which will hold the result of 'generate_number_prefix()'
            let mut temp = String::new();
            // Check if there is a prefix to generate
            if user_input % 1000 > 0 {
                generate_number_prefix(user_input%1000, &mut temp);
                // Add the appropriate suffix to the end of the string
                temp.push_str(SUFFIX[suffix_tracker]);
                // Only add a comma to the string if there is other text present in the final output (i.e. if the input is '100000' we should return "one hundred thousand" with no commas)
                if output.len() > 0 {
                    temp.push_str(", ");
                }
                // Since we are parsing the number 'backwards' we add each new prefix + suffix to the front of the output
                output = temp + &output;
            }
            // Divide user_input by 1000 and increment the suffix tracker
            user_input /= 1000;
            suffix_tracker += 1;
        }

        // Finally, if the original input was negative, add that to the front of the string
        if is_negative {
            output = "negative ".to_owned() + &output
        }

        // Output the result!
        println!("{}", output);

    }
}