//Structs -Used to create custom data types
//traditional struct 
struct Color {
    red: u8,
    green: u8,
    blue: u8
}


pub fn run()
{
    let mut c = Color {
        red:255,
        green:0,
        blue:0
    };

    c.red =200;
    println!("Colors: {} {} {}", c.red, c.green, c.blue);

    let mut d = Color1(255,0,0);
    d.0=200;
    println!("Colors: {} {} {}", d.0,d.1,d.2);

    let mut p = Person::new("John", "Doe");
    println!("Person {} {}" ,p.first_name,p.last_name);
    p.set_last_name("Williams");
    println!("Person {}", p.full_name());

    println!("Person {:?}", p.to_tuple());
    
 
}

//Tuple struct 
struct Color1(u8,u8,u8);

struct Person{
    first_name: String ,
    last_name: String
}

impl Person{

    //consturct the person
    fn new (first: &str, last:&str) -> Person{

        Person{
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }
    
    fn full_name(&self)-> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name (&mut self, last: &str){
        self.last_name= last.to_string();
    }
    //Name to tuple
    fn to_tuple(self)->(String, String){
        (self.first_name, self.last_name)
    }

}