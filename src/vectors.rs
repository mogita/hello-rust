/* Vectors - Resizable arrays. You will probably use vectors much more than arrays 
 */
use std::mem;

pub fn run() {
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    // Get single value
    println!("Singel value: {}", numbers[0]);

    let mut numbers_mut: Vec<i32> = vec![1, 2, 3, 4, 5];
    numbers_mut[2] = 20;
    println!("{:?}", numbers_mut);

    // Add on to vector
    numbers_mut.push(6);
    numbers_mut.push(7);

    // Pop off last value
    numbers_mut.pop();

    // Get array length
    println!("Vector length: {}", numbers_mut.len());

    // Arrays are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers_mut));

    // Get slice from element 0 to element 2
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    // Loop through vector values
    for x in numbers_mut.iter() {
        println!("Number: {}", x);
    }

    // Loop & mutate values
    for x in numbers_mut.iter_mut() {
        *x *= 2;
    }

    println!("Numbers vec: {:?}", numbers_mut);
}
