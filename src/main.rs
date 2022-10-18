mod args;
use args::{Args};
use std::{io,fs};
use std::io::Write;
use std::collections::{HashMap};

fn main() {

    let args = Args::from_cli_args();

    let path = match args.flag_with_key("--file") {
        Some(flag) => match flag.value_as_string() {
            Some(value) => value,
            _ => { panic!("Missing --file value"); }
        },
        _ => { panic!("Missing --file argument"); } 
    };

    let code = match fs::read_to_string(path) {
        Ok(value) => value,
        Err(error) => { panic!("Could not read {}: {}", path, error); }
    };

    let statements: Vec<&str> = code.split(";").map(|s| s.trim()).collect(); 
    let mut variables: HashMap<&str, String> = HashMap::new();

    for statement in statements {
        let statement_items: Vec<&str> = statement.split(":").collect();
        match statement_items[..] {
            [a, b] if a.eq("say") => {
                if b.chars().nth(0).unwrap() == '$' {
                    match variables.get(b) {
                        Some(value) => { print!("{}", value) },
                        None => { panic!("Undefined variable {}", b); },
                    };
                } else {
                    print!("{}", b);
                };
            },
            [a, b] if a.eq("ask") => { 
                let mut line = String::new();
                let len = io::stdin().read_line(&mut line);
                match len {
                    Ok(_) => { variables.insert(b, line); },
                    Err(error) => { panic!("Error reading line: {}", error); }
                }
            },
            [a] if a.eq("exit") => { break; }
            _ => { println!("unknown statement"); }
        };
        io::stdout().flush().unwrap();
    }

}
