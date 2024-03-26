use commands::ClientCmd;
use std::io::{self, Write};

pub fn welcome() {
    println!(
        r#"
    Hey, Welcome to REDIS client!
    
    Example of commands:
        get key01 
        set key02 Hello_World
        exit

    Enjoy!
        "#
    );
}

pub fn get_client_cmd() -> ClientCmd {
    loop {
        print!("> ");
        io::stdout().flush().expect("Cannot flush to stdout");

        let mut buff = String::new();
        io::stdin()
            .read_line(&mut buff)
            .expect("Cannot read from stdin");

        let words: Vec<&str> = buff.split_whitespace().collect();

        match words[..] {
            ["get", key] => {
                return ClientCmd::Get(key.to_string());
            }
            ["set", key, value] => {
                return ClientCmd::Set(key.to_string(), value.to_string());
            }
            ["exit"] => {
                return ClientCmd::Exit;
            }
            _ => {
                println!("E: Incorrect command syntax. Use: [get/set] arg1 arg2");
            }
        };
    }
}
