pub fn run  ()
{
    let mut count =0;
    // loop { // infinite looop

    //     count +=1;
    //     println!("{}", count);
    //     if count == 10{
    //         break   ;
    //     }

    // }

    // while count <= 100 {
    //     if count % 15  == 0 {

    //         println!("fizzbuzz");
    //     }
    //     else if count % 3 == 0 {
    
    //      println!("fizz");        
    //     }
    //     else if count % 5 == 0 {
    //         println!("buzz");
    //     }
    //     else    {
    //         println!("{}", count);
    //     }
    //     count +=1;
    // }

    //range loop
    for x in 0..100{

        if x % 15  == 0 {

            println!("fizzbuzz");
        }
        else if x % 3 == 0 {
    
         println!("fizz");        
        }
        else if x % 5 == 0 {
            println!("buzz");
        }
        else    {
            println!("{}", x);
        }

    }
}