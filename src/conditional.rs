pub fn run()
{
//No ternary operator in Rust 
    let age = 18;
    let chck_id: bool = false;

    if age >= 21 && chck_id {
        println!("Bartender: what would you like to drinnk");

    }
    else if age < 21 && chck_id 
    {

    }
    else if age == 18 || chck_id == false{

    }

    //shorthand 
    let is_ofage = if age >= 21 {true } else {false};
    println!("{}", is_ofage);
}