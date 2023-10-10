use std::io;
use slug::slugify;
use std::env;

fn convert(input: String, mode: &str) -> String {
    match mode {
        "lowercase" => input.to_lowercase(),
        "uppercase" => input.to_uppercase(),
        "no-space" => input.replace(" ", ""),
        "slugify" => slugify(input),
        _ => "UNEXPECTED_ERROR".to_string(),
    }
}

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
        "lowercase" | "uppercase" | "no-space" | "slugify" => println!("Parsing mode: [{}]", mode),
        _ => {
            println!("Unsupported parsing mode: [{}]", mode);
            return
        }
    }


    println!("Now, give me some input:");
    let mut input = String::new();
    _ = io::stdin().read_line(&mut input);
    let input = input.trim();

    let output = convert(input.to_string(), mode);

    print!("Output: {}\n", output);

}
