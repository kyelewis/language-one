pub enum Flag {
    Flag(String),
    FlagWithStringValue(String, String)
} 

impl Flag {
    pub fn value_as_string(&self) -> Option<&String> {
        match &self {
            Flag::Flag(_) => None,
            Flag::FlagWithStringValue(_, v) => Some(v)
        }
    }
}

pub struct Args {
    pub flags: Vec<Flag>,
    pub program_name: Option<String>,
}

impl Args {

    pub fn flag_with_key(&self, flag_key: &str) -> Option<&Flag> {
        for flag in &self.flags {
            match flag {
                Flag::Flag(key) if key.eq(&flag_key) => { return Some(flag) },
                Flag::FlagWithStringValue(key, _) if key.eq(&flag_key) => { return Some(flag) },
                _ => {}
            }
        }
        None
    }

    pub fn from_cli_args() -> Args {
        let mut args = std::env::args().peekable();
        let program_name = args.next();
        let mut flags: Vec<Flag> = Vec::new();

        loop {
            match args.next() {
                // Long Flags
                Some(arg) if arg.starts_with("--") => {

                    // Flag contains an equals, split and use the flag and value
                    if arg.contains("=") {
                      let mut split_arg = arg.split("=");
                      let arg_key = String::from(split_arg.next().unwrap());
                      let arg_value = String::from(split_arg.next().unwrap());

                      flags.push(Flag::FlagWithStringValue(arg_key, arg_value));

                    } else {
                        // Check the next arg to see if it is a recognised flag
                        match args.peek() {

                            // If it isn't, but it exists, use it as a value for the flag 
                            Some(v) if !v.starts_with("--") => {
                                let value = args.next().unwrap();
                                flags.push(Flag::FlagWithStringValue(arg, value));
                            },
        
                            // If it isn't that, then it's a flag without a value
                            _ => { flags.push(Flag::Flag(arg)); }
                        }
                    }
                },
                _ => { break }
            }

        }

        Args {
            program_name,
            flags,
        }
    }
}
