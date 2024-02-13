fn Struct(){
    struct Book {
        title: String,
        author: String,
        publication_year: u32,
    }
    struct Point {
        x: f32,
        y: f32,
    }
    let point = Point { x: 1, y: 2 };
    let another_point = Point { x: -3, y: 4 };
    struct Person {
        name: String,
        age: u32,
    }
     
    fn main() {
        let alice = Person {
            name: String::from("Alice"),
            age: 30,
        };
     
          // Modifying the 'age' field
    alice.age = 31;
    println!("Updated Age: {}", alice.age);;
    }
    fn distance(p1: Point, p2: Point) -> f64 {
        let x_diff = p1.x - p2.x;
        let y_diff = p1.y - p2.y;
        (x_diff * x_diff + y_diff * y_diff).sqrt()
    }
    fn main() {
        let point1 = Point { x: 1.0, y: 1.0 };
        let point2 = Point { x: 4.0, y: 5.0 };
     
        let dist = distance(point1, point2);
        println!("The distance between point1 and point2 is: {}", dist);
        let mid_point = midpoint(point1, point2);
        println!("The midpoint of point1 and point2 is: ({}, {})", mid_point.x, mid_point.y);
    }
    fn midpoint(p1: Point, p2: Point) -> Point {
        let x_mid = (p1.x + p2.x) / 2.0;
        let y_mid = (p1.y + p2.y) / 2.0;
        Point { x: x_mid, y: y_mid }
    }    
}