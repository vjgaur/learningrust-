/*
Primitive Types: --
Integers: u8, i8, u16, u32, i32, u64, i64, u128, i128 (number 
    of bits they take in memory)

    Floats: f32, f64
    Boolean (bool)
    Characters (char)
    Tuples
    Arrays

 Rust is a statically typed language which means that it  
must know the types of all variables at compile time, however
the compiler can usually infer what type we want to use based 
on the value and how we use it. 
*/

pub fn run(){

let x = 1; // default its gonna be i32 type
let y = 2.5; // default its gonna be f64 type

let z: i64 = 44343434343; 

println!("Max i32: {}", std::i32::MAX); //Fin Max Size
println!("Max i64: {}", std::i64::MAX);

let is_active =true;


//boolean expression
let is_greater = 10 > 5;

let a1 = 'a';
let face = '\u{1F600}'; /// emoicons unicode
println!("{:?}", (x,y,z,is_active, is_greater, a1, face));

}