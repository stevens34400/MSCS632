fn print_box(b: Box<i32>) {
    println!("Box contains: {}", b);
    // 'b' is dropped here, and its memory is freed automatically.
}

fn main() {
    // Dynamically allocate an integer on the heap.
    let my_box = Box::new(100);
    
    // This shows borrowing in action.
    // The reference 'borrowed' borrows the value from 'my_box'
    // and does not take ownership of it.
    let borrowed = &my_box;
    println!("Borrowed value: {}", borrowed);
    
    // Ownership is transferred to the function.
    print_box(my_box);
    
    // For the rest of this code 'my_box' is no longer accessible here because its ownership was moved.
}
