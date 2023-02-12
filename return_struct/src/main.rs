
struct Testing {
    alpha: u8,
    beta: u8,
}

fn main() {
    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
    println!("s1: {}", s1);
    println!("s3: {}", s3);

    let new_struct = return_an_initialized_struct();
    println!("struct alpha: {} beta: {}", new_struct.alpha, new_struct.beta);

    let second_struct: Testing = return_a_constructed_struct();
    println!("struct alpha: {} beta: {}", second_struct.alpha, second_struct.beta);
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

fn return_an_initialized_struct() -> Testing {
    Testing {
        alpha: 1,
        beta: 2,
    }
}

fn return_a_constructed_struct() -> Testing {
    let my_struct: Testing = Testing{alpha:21, beta:22};

    my_struct
}