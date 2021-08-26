pub fn run(){

    let numbers: [i32; 5] = [1,2,3,4,5];

    println!("{:?}", numbers);

    //get single vvalue of array
    println!("Single value {}", numbers[0]);

    //arrays ar stack allocated 
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));

    //get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
}