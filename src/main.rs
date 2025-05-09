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

    

}

fn add(x:i32,y:i32)->i32{
    x+y
}

fn multiply(a:f64,b:f64)->f64{
    a*b
}