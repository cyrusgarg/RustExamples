mod utils;
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

}

fn add(x:i32,y:i32)->i32{
    x+y
}

fn multiply(a:f64,b:f64)->f64{
    a*b
}