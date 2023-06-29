enum Status {
    Alive,
    Dead
}

fn main() {

let (firstname, lastname, status) = ("Kekma" , "Time" , Status::Dead);

match status {
    Status::Alive => println!("{} is alive", firstname),
    Status::Dead => println!("{} is dead", firstname),
}

}
