impl Person{
    fn kill(&mut self) {
        self.status = Status::DEAD;
    }
}

enum Status {
    DEAD,
    ALIVE
}

struct Person{
    first_name: String,
    last_name: String,
    age: u16,
    status: Status
}

fn main() {

    let mut person = Person{
        first_name: String::from("John"),
        last_name: String::from("Doe"),
        age: 12,
        status: Status::ALIVE,
    };

    person.kill();

    match person.status {
        Status::ALIVE =>println!("{} {} is alive", person.first_name, person.last_name),
        Status::DEAD =>println!("{} {} is dead", person.first_name, person.last_name)
    }
}
