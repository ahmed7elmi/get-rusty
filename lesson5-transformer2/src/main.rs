mod transformers;
use std::io;
use std::env;
use transformers::transform;

/// Application's entry point.
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        eprintln!("Only argument is allowed (for now)!");
        return
    }
    else if args.len() < 2 {
        eprintln!("Missing argument!!");
        return
    }

    let mode: &str = &args[1];

    match mode {
        "lowercase" | "uppercase" | "no-space" | "slugify" | "title" | "pascal" | "password" | "csv" => eprintln!("Parsing mode: [{}]", mode),
        _ => {
            eprintln!("Unsupported parsing mode: [{}]", mode);
            return
        }
    }


    loop {
        println!("Enter input to transform:");
        let mut input = String::new();

        if mode == "csv" {

            loop { // read all lines
                let mut line = String::new();
                _ = io::stdin().read_line(&mut line);
                let line = line.trim();
                if line == "" {
                    // end of Csv input reached.
                    break;
                }
                input.push_str(line);
                input.push('\n');
            }

            println!("{}", input)

        }
        else {
            _ = io::stdin().read_line(&mut input);
        }
        let input = input.trim();

        match transform(input.to_string(), mode) {
            Ok(output) => println!("Output:\n{output}"),
            Err(e) => eprintln!("An error occurred: {e}"),
        }

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
