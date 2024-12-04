use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn eval_mul(input: TokenStream) -> TokenStream {
    // Extract the string literal from the input
    let input = input.to_string();
    let trimmed = input.trim();

    // Ensure the input is a valid string literal
    if !trimmed.starts_with('"') || !trimmed.ends_with('"') {
        return TokenStream::from(quote! {
            compile_error!("Input must be a string literal.");
        });
    }

    // Remove the surrounding quotes
    let input_str = &trimmed[1..trimmed.len() - 1];

    let mut sum_part_01 = 0;
    let mut sum_part_02 = 0;
    let mut skip: bool = false;

    let mut chars = input_str.chars().peekable();

    // Tokenize and parse the string manually
    // könnte man noch schön auslagern, aber kb mehr
    while let Some(ch) = chars.next() {
        if ch == 'd' && chars.peek() == Some(&'o') {
            chars.next();
            if chars.peek() == Some(&'(') {
                chars.next();
                if chars.peek() == Some(&')') {
                    chars.next();
                    skip = false;
                }
            } else if chars.peek() == Some(&'n') {
                chars.next();
                if chars.peek() == Some(&'\'') {
                    chars.next();
                    if chars.peek() == Some(&'t') {
                        chars.next();
                        if chars.peek() == Some(&'(') {
                            chars.next();
                            if chars.peek() == Some(&')') {
                                chars.next();
                                skip = true;
                            }
                        }
                    }
                }
            }
        }

        // Look for the `mul` keyword
        if ch == 'm' && chars.peek() == Some(&'u') {
            chars.next(); // Consume 'u'
            if chars.peek() == Some(&'l') {
                chars.next(); // Consume 'l'

                // Check for an opening parenthesis
                if chars.peek() == Some(&'(') {
                    chars.next(); // Consume '('
                    if let Some((a, b)) = parse_mul_args(&mut chars) {
                        sum_part_01 += a * b;
                        if !skip {
                            sum_part_02 += a * b;
                        }
                    }
                }
            }
        }
        // Skip any other characters (garbage handling)
    }

    // Generate code to declare and initialize both results
    // Macro does not return anything, these variables will just be there
    // after calling the macro, like magic, lul
    TokenStream::from(quote! {
        let sum_part_01 = #sum_part_01;
        let sum_part_02 = #sum_part_02;
    })
}

// Parse the arguments of `mul(a, b)`
fn parse_mul_args(chars: &mut std::iter::Peekable<std::str::Chars>) -> Option<(u32, u32)> {
    // Parse the first number
    let a = parse_number(chars)?;
    // Consume the comma
    if chars.next()? != ',' {
        return None;
    }
    // Parse the second number
    let b = parse_number(chars)?;
    // Consume the closing parenthesis
    if chars.next()? != ')' {
        return None;
    }
    Some((a, b))
}

// Parse a number from the character stream
fn parse_number(chars: &mut std::iter::Peekable<std::str::Chars>) -> Option<u32> {
    let mut num = String::new();
    while let Some(&ch) = chars.peek() {
        if ch.is_ascii_digit() {
            num.push(ch);
            chars.next(); // Consume the digit
        } else {
            break;
        }
    }
    num.parse::<u32>().ok()
}
