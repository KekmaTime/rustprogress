use std::io;
use std::io::Write;

fn main() {
    loop { 
        print!("> ");
        let _ = io::stdout().flush();
        let mut input = String::new();

        let _ = io::stdin().read_line(&mut input);

        println!("You said: {}", input);

        if input.trim_end().eq_ignore_ascii_case("exit") {
            break;
        }
    }
}

