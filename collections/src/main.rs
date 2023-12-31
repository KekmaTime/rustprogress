fn main() {

    let mut vector : Vec<u8> = Vec::new();

    //or

    let mut vector2 : Vec<u8> = Vec::new();

    vector.push(1);
    vector.push(2);
    vector.push(3);
    vector.push(4);
    vector.push(5);

    for data in vector.iter() {
        println!("{}", data);
    }
    vector.insert(3,99);
    vector.remove(2);

    for data in vector.iter() {
        println!("{}", data);
    }

    let i = vector.pop();

    match i {
        Some(d) => println!("{}", d),
        None => println!("Not found"),
    }

    let result = vector.contains(&3);
    let resutl2 = vector.contains(&99);

    println!("{}", result);
    println!("{}", resutl2);
}
