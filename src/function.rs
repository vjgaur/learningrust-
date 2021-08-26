pub fn run  ()
{
    greetings("hello", "jane");
    //bind function values to variabale
    let get_sum = add(5,5);
    println!("Sum: {}", get_sum);

    //Closure 
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2:i32| n1+n2 +n3;
    println!("c Sum: {}", add_nums(3,3));
}

fn greetings (greet:&str, name: &str){

    println!("{} {} nice to meet you", greet, name);
}

// function that returns some value 
fn add(n1: i32, n2: i32) -> i32{

    n1+n2
}