fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &x; // z is an immutable reference to x

    *y = 10; // Modify x through y
    println!("x = {}", x); // Output: x = 10
    //println!("z = {}", *z); // This line will cause a compile-time error because you can not read an immutable reference if a mutable reference exist
}