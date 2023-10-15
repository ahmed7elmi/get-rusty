use std::io;
use slug::slugify;
use std::env;

/// Transformer struct offers a methodology to transorm input `String` into 
/// different ways by setting multiple calls of its functions in the form of
/// a pipeline.
struct Transformer(String);

impl Transformer {

    /// A wrapper around the `to_lowercase` function.
    fn to_lower(mut self) -> Transformer {
        self.0 = self.0.to_lowercase();
        self
    }

    /// A wrapper around the `to_uppercase` function.
    fn to_upper(mut self) -> Transformer {
        self.0 = self.0.to_uppercase();
        self
    }

    /// Removes all spaces from the input string.
    fn with_no_spaces(mut self) -> Transformer {
        self.0 = self.0.replace(" ", "");
        self
    }

    /// A wrapper around the slugify function.
    fn slugify(mut self) -> Transformer {
        self.0 = slugify(self.0);
        self
    }

    /// Converts the first letter of each word in the string to upper case. 
    fn to_pascal_case(mut self) -> Transformer {
        let chars: Vec<char> = self.0.chars().collect();
        let mut output = String::new();
        let mut new_word = true;

        for mut c in chars {
            if new_word {
                c = c.to_ascii_uppercase();
            }
            new_word = c == ' ';
            output.push(c);
        }

        self.0 = output;

        self
    }

    /// Replace specific characters in the input string (if found) with another 
    /// ascii letter to make the whole input looks like a password.
    fn passwordify(mut self) -> Transformer {
        self.0 = self.0.replace("a", "@")
            .replace("o", "0")
            .replace("s", "$")
            .replace("i", "!");
        self
    }

    /// Returns the input string with its current transformed (or not transformed) state.
    fn value(self) -> String {
        self.0
    }
}

/// A helper method that calls the `Transformer` with different settings based 
/// on the `mode` parameter to transform the `input` string.
fn transform(input: String, mode: &str) -> String {
    let t = Transformer(input);

    match mode {
        "lowercase" => t.to_lower().value(),
        "uppercase" => t.to_upper().value(),
        "no-space" => t.with_no_spaces().value(),
        "slugify" => t.slugify().value(),
        "title" => t.to_pascal_case().value(),
        "pascal" => t.to_pascal_case().with_no_spaces().value(),
        "password" => t.to_pascal_case().with_no_spaces().passwordify().value(),
        _ => "UNEXPECTED_ERROR".to_string(),
    }
}

/// Application's entry point.
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Only argument is allowed (for now)!");
        return
    }
    else if args.len() < 2 {
        println!("Missing argument!!");
        return
    }

    let mode: &str = &args[1];

    match mode {
        "lowercase" | "uppercase" | "no-space" | "slugify" | "title" | "pascal" | "password" => println!("Parsing mode: [{}]", mode),
        _ => {
            println!("Unsupported parsing mode: [{}]", mode);
            return
        }
    }


    loop {
        println!("Enter input to transform:");
        let mut input = String::new();
        _ = io::stdin().read_line(&mut input);
        let input = input.trim();

        let output = transform(input.to_string(), mode);

        println!("Output: {output}");
        println!();
        
        println!("Do you want to transform more strings (Y/N)?");
        let mut choice = String::new();
        _ = io::stdin().read_line(&mut choice);
        let choice = choice.trim();
        if choice == "N" || choice == "n" {
            break;
        }
    }
}
