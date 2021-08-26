pub fn run(){
//vectors are resizable arrays 

    let mut numbers: Vec<i32> = vec![1,2,3,4,5];

    numbers.push(6);
    println!("{:?}", numbers);
    numbers.pop();
    //get single vvalue of array
    println!("Single value {}", numbers[0]);

    //arrays ar stack allocated 
    println!("Vector occupies {} bytes", std::mem::size_of_val(&numbers));

    //get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    // loop through vector values 
    for x in numbers.iter(){
        println!("Number: {}", x);
    }

    //loop & mutate the vector values
    for x in numbers.iter_mut(){
        *x*=2;
    }
    println!("Numbers New Vec Values : {:?}", numbers);
}