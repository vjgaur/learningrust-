//Primitive str = Immutable fixed-length string somewehre in memory
// String = Growable heap allocated data structure use when you need to modify or own string data


pub fn run()
{
let hello = "hello"; // immutable
let mut hello1 = String::from("hello1"); //growabble 



hello1.push('w');
hello1.push_str("this is string");

println!("Lenght: {}, Capacity: {}", hello.len(), hello1.capacity());
println!("{} is immutbale, {} is growable", hello,hello1);
println!("Is Empty: {}", hello1.is_empty());
//loop theough string by whitespace
for word in hello1.split_whitespace(){
    println!("{}", word);
}

//Create string with capacity 
let mut s= String::with_capacity(10);
s.push('a');
s.push('b');

//assertion testing 
assert_eq!(2,s.len()); // it only show error when it fails otherwise it doesnt show antyhing if test pass
assert_eq!(12,s.capacity());

}
