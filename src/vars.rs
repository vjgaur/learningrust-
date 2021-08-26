//Variables are immutable by default
pub fn run()
{
    let name ="Vijendr";
    let mut age = 37;

    println!("My Name is {} I am {}", name, age);
    age = 38;
    println!("My Name is {} I am {}", name, age);
    //define constant 
    const ID: i32 = 001;
    println!("ID: {}", ID);

    //Assign multi variable at once 

    let (my_name, my_age) = ("Vijay",37);
    println!("{} is {}", my_name, my_age);
}