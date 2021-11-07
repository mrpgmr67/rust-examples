// Variables holding the values of the variables
// Variables are immutable by default
// Variables are mutable if mut is used
// Rust is a block-scoped language

pub fn run(){
    let name = "Brad";
    let mut age = 37;
    println!("My name is {} and I am {}", name, age);
    age = 38;
    println!("My name is {} and I am {}", name, age);
    
    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

}