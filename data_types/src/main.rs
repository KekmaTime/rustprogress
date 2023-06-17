fn main() {
    let int = -20;
    let uint = 20;

    let f32 = 8.2;
    let f64 = 31.4;

    let bool = true;
    
    let char = 'A';

    let array = [1, 2, 3, 4, 5];
    let array1 = [0; 1000];

    println!("{}" , array[2]);

    let tuple = ("Charles" , "Dickens" , 1812);

    println!("{} {} was born in {}.", tuple.0, tuple.1, tuple.2);

    let (first_name , last_name, dob) = tuple;

    println!("{} {} was born in {}.", tuple.0, tuple.1, tuple.2);

    let slice = "Charles Dickens";

    let str1 = slice.to_string();

    //or 

    let str2 = "Charles Dickens".to_string();

    let str3 = String::from("Charles Dickens");

    let str4 = String::from("Charles Dickens");

    


}

