fn main() {
    let mut x = 5;
    { //Creating a new scope
        let y = &mut x; // y is a mutable reference to x
        *y = 10; // Modify x through y
    }
    let z = &x; // z is an immutable reference to x. This is now valid because the mutable reference y is out of scope.
    println!("x = {}", x); // Output: x = 10
    println!("z = {}", *z); // Output: z = 10
} 