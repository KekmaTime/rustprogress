fn main() {
    

    let closure = |name: &str| -> String{
        format!("Hello , {}", name)
    };

    let message = closure("Rasta");
    println!("{}", message);
}
