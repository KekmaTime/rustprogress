fn main() {

    let rectangle = Rectangle{
        length: 20,
        width: 15
    };

    let area = rectangle.calculate_area();

    println!("Length: {}, Width: {}, Area: {}", rectangle.length, rectangle.width,area);
}

struct Rectangle{
    length: u16,
    width: u16
}

impl Rectangle {
    fn calculate_area(&self) -> u16 {
        self.length*self.width
    }
}
