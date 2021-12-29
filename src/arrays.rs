/* Arrays - Fixed list where elements are the same data types
 *
 */

use std::mem;

pub fn run() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    // Get single value
    println!("Singel value: {}", numbers[0]);

    let mut numbers_mut: [i32; 5] = [1, 2, 3, 4, 5];
    numbers_mut[2] = 20;
    println!("{:?}", numbers_mut);

    // Get array length
    println!("Array length: {}", numbers_mut.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers_mut));

    // Get slice from element 0 to element 2
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
}
