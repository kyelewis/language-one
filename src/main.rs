mod args;
use args::Args;
use std::{io,fs,process};
use std::io::Write;
use std::collections::HashMap;

#[derive(Debug,Clone)]
enum Type {
    String(String),
    Number(i32)
}

#[derive(Debug,Clone)]
enum Token {
    Identifier(Type),
    Literal(Type),
    EndOfStatement,
    EndOfFile,
    Whitespace
}

#[tokio::main]
async fn main() {

    let args = Args::from_cli_args();

    let path = match args.flag_with_key("--file") {
        Some(flag) => match flag.value_as_string() {
            Some(value) => value,
            _ => { panic!("Missing --file value"); }
        },
        _ => { panic!("Missing --file argument"); } 
    };

    let code_string = match fs::read_to_string(path) { 
        Ok(value) => value,
        Err(error) => { panic!("Could not read {}: {}", path, error); }
    };

    let mut code_characters = code_string.chars().peekable();

    // tokenise
    let mut tokens: Vec<Token> = Vec::new();

    loop {
        
        let token = match code_characters.next() {
            // 0-9, read until non-number character
            Some(character) if character.is_ascii_digit() => {
                let mut number = String::from(character);
                loop {
                    match code_characters.peek() {
                        Some(c) if c.is_ascii_digit() => {
                            let next_character = code_characters.next().unwrap();
                            number.push(next_character);
                        },
                        _ => { break; }
                    };
                };
                match number.parse() {
                    Ok(number) => Some(Token::Literal(Type::Number(number))),
                    Err(error) => panic!("Could not parse number: {}", error),
                }
            },
            // ` string literal, Read until next ` and store a StringLiteral
            Some(character) if character.is_ascii_whitespace() => {
                loop {
                    match code_characters.peek() {
                        Some(character) if character.is_ascii_whitespace() => { },
                        _ => { break; }
                    };
                };
                Some(Token::Whitespace)
            },
            // semicolon ends a statement
            Some(character) if ';'.eq(&character) => Some(Token::EndOfStatement),
            // ` is a string literal, read until the next ` 
            Some(character) if '`'.eq(&character) => {
                let mut string_literal = String::new();   // Don't use the `
                loop {
                    match code_characters.peek() {
                        Some(c) if !('`'.eq(&c)) => {
                            let next_character = code_characters.next().unwrap();
                            string_literal.push(next_character);
                        },
                        _ => { 
                            // Eat the last `
                            let _ = code_characters.next();
                            break;
                        }
                    };
                };
                Some(Token::Literal(Type::String(string_literal)))
            },
            // anything else is a valid identifier, read until space or semicolon
            Some(character) => {
                let mut identifier = String::from(character);
                loop {
                    match code_characters.peek() {
                        Some(c) if !c.is_ascii_whitespace() && !';'.eq(&c) => {
                            let next_character = code_characters.next().unwrap();
                            identifier.push(next_character);
                        },
                        _ => { break; }
                    };
                };
                Some(Token::Identifier(Type::String(identifier)))
            },
            None => Some(Token::EndOfFile), 
        };

        if let Some(token) = token {
          if let Token::EndOfFile = token {
            tokens.push(token);
              break;
          } else {
              tokens.push(token);
          }
        };

    };

//    println!("Tokens:\n{:#?}", tokens);

    // parse - this could probably be done with fold if i were a better rust programmer
    let mut statements: Vec<Vec<Token>> = Vec::new();

    let mut tokens = tokens.iter();

    let mut next_statement: Vec<Token> = Vec::new();
    loop {
        match tokens.next() {
           Some(Token::EndOfStatement) => {
               statements.push(next_statement);
               next_statement = Vec::new();
           },
           None | Some(Token::EndOfFile) => {
               break;
           },
           Some(Token::Whitespace) => {},
           Some(token) => {
               next_statement.push(token.clone());
           },
        };
    };

 //   println!("Statements:\n{:#?}", statements);

    // Run
   
    let mut variables: HashMap<String, Type> = HashMap::new();

    for statement in statements {

        match statement.get(0) {
            Some(Token::Identifier(Type::String(value))) => {
                // Function call
                
                match value.as_str() {
                    "say" => {
                        // print to console
                        match statement.get(1) {
                            Some(Token::Literal(Type::String(value))) => {
                                print!("{}", value);
                            },
                            Some(Token::Literal(Type::Number(value))) => {
                                print!("{}", value);
                            },
                            Some(Token::Identifier(Type::String(value))) => {
                                match variables.get(value) {
                                    Some(Type::String(value)) => {
                                        print!("{}", value);
                                    },
                                    Some(Type::Number(value)) => {
                                        print!("{}", value);
                                    },
                                    _ => {
                                        print!("(undefined)");
                                    },
                                };
                            },
                            _ => {
                                panic!("Unexpected value after 'say'");
                            },
                        };
                    },
                    "ask" => {
                        // ask for input 
                        match statement.get(1) {
                            Some(Token::Identifier(Type::String(value))) => {
                                // Ask for input
                                let mut line = String::new(); 
                                let _ = io::stdin().read_line(&mut line);
                                    
                                // @todo could be a number?
                                variables.insert(String::from(value), Type::String(String::from(line.trim_end())));
                            },
                            _ => {
                                panic!("Unexpected value after 'ask'");
                            },
                        };
                    },
                    "http_get" => {
                        // get from http and store in a variable
                        match statement.get(1) {
                            Some(Token::Literal(Type::String(value))) => {

                                if let Ok(response) = reqwest::get(value).await.expect("http get failed").text().await {
                                    match statement.get(2) {
                                        Some(Token::Identifier(Type::String(value))) => {
                                            variables.insert(String::from(value), Type::String(response));
                                        },
                                        _ => { panic!("No identifier in second arg for http-get"); },
                                    };
                                };

                            },
                            _ => { panic!("No URL in first arg for http_get"); },
                        };
                    },
                    "exit" => {
                        match statement.get(1) {
                            Some(Token::Literal(Type::Number(value))) => {
                                process::exit(*value);
                            },
                            _ => {
                                process::exit(0);
                            },
                        };
                    },
                    _ => {},
                };
            },
            _ => {},
        }

        io::stdout().flush().unwrap();
    }

}

