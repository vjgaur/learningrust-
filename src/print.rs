pub fn run()
{
    println!("Hello from print.rs file");

    println!("{} is from {}", "Vijayendr", "Gaur");

    println!("{0} is from {1} and {0} likes to {2}", "Vijayendra","Pune","Code");
    
    println!("{name} likes to play {activity}", name="John", activity="Cricket");


//placeholder traits
println!("Binary:  {:b} Hex: {:x} Octal: {:o}", 10,10,10);

//placeholder for debug trait 

println!("{:?}", (12, true, "hello")); //array or tuple

//Basic math
println!("10+10 = {} ", 10+10);

}
