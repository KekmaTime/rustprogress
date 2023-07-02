fn main() {
    
    for i in 0..10 {
        println!("{}", i);
    }

    for i in 0..=10{
        println!("{}", i);
    }

    let array = [1, 2, 3, 4, 5,];
    
    for index in 0..array.len() {
        println!("{}", array[index]);
    }

    for data in array.iter() {
        println!("{}", data);
    }

    let mut counter = 0;
    while counter <=10{
        println!("{}", counter);
        counter += 1;
    }

    loop {
        println!("{}", counter);
        if counter == 10{
            break;
        }
        counter += 1;
    }

}
