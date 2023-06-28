use std::collections::HashMap;
use std::collections::BTreeMap;

fn main() {
    let mut map: HashMap<u8, &str> = HashMap::new();

    map.insert(1, "one");
    map.insert(2, "two");
    map.insert(3, "three");
    map.insert(4, "four");
    map.insert(5, "five");

    for kvp in  map.iter() {
        println!("{}: {}", kvp.0, kvp.1);
    }

    let mut map1: BTreeMap<u8, &str> = BTreeMap::new();

    map1.insert(1, "one");
    map1.insert(2, "two");
    map1.insert(3, "three");
    map1.insert(4, "four");
    map1.insert(5, "five");

    if !map.contains_key(&1){
        map.insert(1, "kekma");
    }

    for kvp in  map1.iter() {
        println!("{}: {}", kvp.0, kvp.1);
    }

    let value = map.get(&3);
    match value {
        Some(v) => println!("{}", v),
        None => println!("not found")
    }


}
