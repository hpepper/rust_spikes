use std::fmt;

/**
 * Shows the ownership of variables in Rust, for three different types: int, string and struct.
 */

// ===================================================== INT OWNERSHIP

fn increase_value(x: &mut i32){
    *x += 1;
}

fn show_int_in_sub_function(x: i32){
    println!("x in sub function: {}", x);
}

fn int_ownership() {
    println!("----- int ownership");
    let mut alpha: i32 = 5;
    let beta: i32 = alpha; // I think this does a simple copy
    println!("alpha: {}, beta: {}", alpha, beta);
    increase_value(&mut alpha);
    println!("alpha after increase: {}", alpha);
    show_int_in_sub_function(beta);
}

// ===================================================== STRING OWNERSHIP

fn append_to_string(s: &mut String){
    s.push_str(" world");
}

fn show_string_in_sub_function(s: &String){
    println!("s in sub function: {}", s);
}

fn string_ownership() {
    println!("----- String ownership");
    let mut alpha: String = String::from("Hello");
    let beta: String = alpha.clone(); // The clone is required, without the clone, the alpha will be moved to beta and alpha is then unavailable.
    println!("alpha: {}, beta: {}", alpha, beta);
    append_to_string(&mut alpha);
    println!("alpha after increase: {}", alpha);
    show_string_in_sub_function(&beta); // This is a reference, so the ownership is not moved to the sub function
    println!("beta after call: {}", beta);

}

// ===================================================== STRUCT OWNERSHIP

// The struct needs to derive the Clone trait to be able to clone it.
#[derive(Clone)]
struct MyStruct {
    text: String,
    x: i32,
    y: i32,
}

impl fmt::Display for MyStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MyStruct {{ text: {}, x: {}, y: {} }}", self.text, self.x, self.y)
    }
}

fn append_to_struct_string(s: &mut MyStruct){
    s.text.push_str(" world");
    s.x += 1;
}

fn show_struct_in_sub_function(s: &MyStruct){
    println!("s in sub function: {}", s);
}

fn struct_ownership() {
    println!("----- Struct ownership");
    // The structure var has to be mutable to be able to change the values.
    let mut alpha: MyStruct = MyStruct {
        text: String::from("Hello"),
        x: 5,
        y: 10,
    };
    let beta = alpha.clone();
    println!("alpha: {}, beta: {}", alpha, beta);
    append_to_struct_string(&mut alpha);
    println!("alpha after string change: {}", alpha);
    show_struct_in_sub_function(&beta); // This is a reference, so the ownership is not moved to the sub function
    println!("beta after call: {}", beta);
}

fn main() {
    println!("Hello, world!");
    int_ownership();
    string_ownership();
    struct_ownership();
}
