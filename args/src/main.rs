use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 3{
        println!("Not enough arguments!");
        show_usage();
        return;
    }
    for arg in args.iter() {
        println!("{}", arg);
    }

}

fn show_usage() {
    println!("Usage: app.exe <arg1> <arg2>");
}

