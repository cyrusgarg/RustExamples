mod utils;
mod even;
mod fib;
use fib::fib;
use crate::even::check_even;
use std::fmt::Debug;

struct User{
    active:bool,
    username:String,
    email:String,
    sign_in_count: u64,
}

struct Rectangle {
    width: u32,
    height: u32,
}
impl Debug for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rectangle {{ width: {}, height: {} }}", self.width, self.height)
    }
    // fn area(&self) -> u32 {
    //     self.width * self.height
    // }
}

struct NoShape;
impl NoShape {
    fn area(&self) -> u32 {
        0
    }
}

struct Point<T> {
    x: T,
    y: T,
}

enum Shape {
    Circle(f64),
    Square(f64),
}


fn main() {
    println!("Hello, world!");
    utils::greet("Cyrus");

    let age:i32=25;
    let year:i32=2025;
    let big_number:u64=1000000;

    let pi:f64=3.14159;
    let temp:f64=36.6;

    println!("Age: {}",age);
    println!("Year: {}",year);
    println!("Big number: {}",big_number);
    println!("Pi: {}",pi);
    println!("Body temperature: {}",temp);

    let sum=add(10,15);
    let product =multiply(3.5,2.0);

    println!("Sum: {}",sum);
    println!("Product: {}",product);

    let greeting="Hello, world!";
    println!("{}",greeting);

    let mut name=String::from("Rust");
    println!("Initial:{}",name);

    name.push_str("Programming");
    println!("After push: {}",name);

    let replaced=name.replace("Rust","Systems");
    println!("Replaced: {}",replaced);

    println!("length: {}",name.len());

    for ch in name.chars(){
        print!("{}",ch);
    }

    println!();
    let number=42;
    let num_str=number.to_string();
    println!("Number as string: {}",num_str);
    update_string();

    let mut my_string = String::from("Hello, Rust!");
    my_string=takes_ownership(my_string);
    println!("{}", my_string); // This will cause an error because ownership has been moved
    // Borrowing example
    borrowing_example(&mut my_string);
    println!("After borrowing: {}", my_string); // This is valid because we borrowed, not moved

    let user1=User{
        active:true,
        username: String::from("Cyrus"),
        email: String::from("cyrus1@ualberta.ca"),
        sign_in_count:1,
    };
    println!("User 1 username: {:?}",user1.username);

    let rect= Rectangle {
        width: 30,
        height: 50,
    };
    // println!("Rectangle area: {}", rect.area());
    println!("Rectangle: {:?}", rect); // Using the Debug trait to print the rectangle

    let rect=NoShape;
    println!("NoShape area: {}", rect.area()); // Calling the area method on NoShape

    let circle = Shape::Circle(5.0);
    let square = Shape::Square(4.0);
    println!("Area of the circle: {}",calculate_area(circle));
    println!("Area of square: {}",calculate_area(square));

    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.5, y: 2.5 };
    println!("Integer Point: ({}, {})", integer_point.x, integer_point.y);
    println!("Float Point: ({}, {})", float_point.x, float_point.y);

    // let result: Result<String,Error> = fs::read_to_string("file.txt");
    // match result {
    //     Result::Ok(content) => println!("File content: {}", content),
    //     Result::Err(e) => println!("Error reading file: {:?}", e),
    // }

    let s = "Hello, world!";
    let res = find_first_a(s); 
    match res {
        Some(index) => println!("First 'a' found at index: {}", index),
        None => println!("No 'a' found in the string."),
    }  
    let num = 32;
    println!("Is Number Even? {}",check_even(num));    

    let fib_num = 10;
    let fib_result = fib(fib_num);
    println!("Fibonacci of {} is {}", fib_num, fib_result);
}



fn find_first_a(s: &str) -> Option<usize> {
    for (i, c) in s.chars().enumerate() {
        if c == 'a' {
            return Some(i);
        }
    }
    None
}

fn takes_ownership(s:String)->String{
    println!("Taking ownership of: {}",s);
    s // Ownership is moved here
}

fn borrowing_example(s:&mut String) {
    s.push_str(" - borrowed");
    println!("Borrowing: {}", s);
}

fn add(x:i32,y:i32)->i32{   
    x+y
}

fn multiply(a:f64,b:f64)->f64{
    a*b
}

fn update_string() {
    let mut my_string = String::from("Hello");
    println!("Capacity: {}", my_string.capacity());
    println!("Length: {}", my_string.len());
    println!("Pointer: {:p}", my_string.as_ptr());
    my_string.push_str(", Rust!");
    println!("Updated string: {}", my_string);
}

fn calculate_area(shape: Shape)->f64 {
    let ans=match shape {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Square(side) => side * side,
        
    };

    return ans;
}


