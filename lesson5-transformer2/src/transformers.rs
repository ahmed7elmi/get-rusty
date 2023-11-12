pub mod csv;

use std::fmt;
use std::error::Error;
use slug::slugify;
use crate::transformers::csv::Csv;

/// Transformer struct, used to transorm input `String` into another by
/// chaining multiple calls of its functions to form a pipeline for the
/// transformation.
struct Transformer {
    data: String,
}

impl fmt::Display for Transformer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}

impl Transformer {
    /// Transforms the entire string input into a `Csv` table.
    fn to_csv(mut self) -> Result<Transformer, Box<dyn Error>> {
        let mut header_vec: Vec<String> = vec![];
        let mut rows_matrix: Vec<Vec<String>> = vec![];
        let mut first_line = true;
        let mut column_count: usize = 0;
        let lines: Vec<&str> = self.data.split('\n').collect();
        let mut line_number = 0;
        for line in lines {
            let line_splitted = line.split(',').map(|s| s.to_string()).collect();
            if first_line {
                header_vec = line_splitted;
                column_count = header_vec.len();
                first_line = false;
            }
            else {
                if line_splitted.len() != column_count {
                    return Err(format!("Mismatched columns count at line {line_number}, (all rows should have {column_count} columns)").into());
                }
                rows_matrix.push(line_splitted);
            }
            line_number += 1;
        }

        let csv = Csv::new(header_vec, rows_matrix, '=');
        self.data = csv.to_string();


        Ok(self)
    }

    /// A wrapper around the `to_lowercase` function.
    fn to_lower(mut self) -> Result<Transformer, Box<dyn Error>> {
        self.data = self.data.to_lowercase();
        Ok(self)
    }

    /// A wrapper around the `to_uppercase` function.
    fn to_upper(mut self) -> Result<Transformer, Box<dyn Error>> {
        self.data = self.data.to_uppercase();
        Ok(self)
    }

    /// Removes all spaces from the input string.
    fn with_no_spaces(mut self) -> Result<Transformer, Box<dyn Error>> {
        self.data = self.data.replace(" ", "");
        Ok(self)
    }

    /// A wrapper around the slugify function.
    fn slugify(mut self) -> Result<Transformer, Box<dyn Error>> {
        self.data = slugify(self.data);
        Ok(self)
    }

    /// Converts the first letter of each word in the string to upper case. 
    fn to_pascal_case(mut self) -> Result<Transformer, Box<dyn Error>> {
        let chars: Vec<char> = self.data.chars().collect();
        let mut output = String::new();
        let mut new_word = true;

        for mut c in chars {

            // return error if character is not ascii
            if !c.is_ascii() {
                return Err(format!("Letters has to be ascii, {c} detected!").into());
            }
            if new_word {
                c = c.to_ascii_uppercase();
            }
            new_word = c == ' ';
            output.push(c);
        }

        self.data = output;

        Ok(self)
    }

    /// Replace specific characters in the input string (if found) with another 
    /// ascii letter to make the whole input looks like a password.
    fn passwordify(mut self) -> Result<Transformer, Box<dyn Error>> {
        self.data = self.data.replace("a", "@")
            .replace("o", "0")
            .replace("s", "$")
            .replace("i", "!");
        Ok(self)
    }
}

/// A helper method that calls the `Transformer` with different settings based 
/// on the `mode` parameter to transform the `input` string.
pub fn transform(input: String, mode: &str) -> Result<String, Box<dyn Error>> {
    let t = Transformer { data: input };

    match mode {
        "lowercase" => Ok(t.to_lower()?.to_string()),
        "uppercase" => Ok(t.to_upper()?.to_string()),
        "no-space" => Ok(t.with_no_spaces()?.to_string()),
        "slugify" => Ok(t.slugify()?.to_string()),
        "title" => Ok(t.to_pascal_case()?.to_string()),
        "pascal" => Ok(t.to_pascal_case()?.with_no_spaces()?.to_string()),
        "password" => Ok(t.to_pascal_case()?.with_no_spaces()?.passwordify()?.to_string()),
        "csv" => Ok(t.to_csv()?.to_string()),
        _ => Err("UNEXPECTED_ERROR".into()),
    }
}