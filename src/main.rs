mod utils;
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